//! Comandos de plataforma; contratos de apps/cli/src/commands da referência.

mod batch;
mod generate;
use crate::{
    args::Args,
    input,
    output::{self, Output},
};
use br_validators::{self as br, DocumentType, PlatformOptions};
use serde_json::{json, Value};

pub fn recognizes(command: &str) -> bool {
    matches!(
        command,
        "detect" | "sanitize" | "mask" | "compare" | "batch" | "diff" | "generate" | "list"
    )
}

pub fn run(command: &str, args: &Args) -> Output {
    if command == "list" {
        return Output {
            stdout: [
                "cnpj",
                "cpf",
                "cep",
                "telefone",
                "cnh",
                "renavam",
                "titulo-eleitor",
                "processo-judicial",
                "rg",
                "nfe-chave",
                "brcode",
                "placa",
                "pis-pasep",
                "pix",
                "boleto",
                "cartao",
                "ie",
            ]
            .map(str::to_owned)
            .to_vec(),
            ..Output::default()
        };
    }
    let options = PlatformOptions {
        uf: args
            .uf
            .as_ref()
            .and_then(|uf| uf.to_uppercase().parse().ok()),
    };
    if command == "detect" {
        return detect(args, &options);
    }
    let Some(ty) = args.args.first() else {
        return Output::usage(format!("Missing type. Usage: {command} <type>"));
    };
    if command == "generate" {
        return generate::run(ty, args, &options);
    }
    let parsed = ty.parse::<DocumentType>();
    let supported = parsed.as_ref().is_ok_and(|ty| match command {
        "sanitize" => !matches!(
            ty,
            DocumentType::Brcode | DocumentType::Rg | DocumentType::ProcessoJudicial
        ),
        "mask" => *ty != DocumentType::Brcode,
        _ => true,
    });
    if !supported {
        return Output::usage(format!("Unsupported {command} type: {ty}"));
    }
    let Ok(ty) = parsed else {
        return Output::usage(format!("Unsupported {command} type: {ty}"));
    };
    if command == "batch" {
        return batch::run(ty, args, &options);
    }
    if matches!(command, "compare" | "diff") {
        return compare(command, ty, args, &options);
    }
    let input = match input::resolve(args.args.get(1).map(String::as_str), args.file.as_deref()) {
        Ok(Some(value)) => value,
        Ok(None) => return Output::usage("Missing value. Pass an argument or use --file."),
        Err(message) => return Output::usage(message),
    };
    if command == "mask" {
        let result = match br::mask(&input, ty, &options) {
            Ok(value) => json!({"ok":true,"formatted":value}),
            Err(error) => json!({"ok":false,"code":error.code,"message":error.message}),
        };
        return output::format(result, args.json, args.quiet);
    }
    let result = match br::sanitize(&input, ty, &options) {
        Ok(result) => json!({"ok":true,"value":result.value,"fixes":result.fixes}),
        Err(error) => json!({"ok":false,"code":error.code,"message":error.message}),
    };
    let valid = result["ok"] == true;
    let mut out = render(&result, u8::from(!valid), args);
    if args.json || args.quiet {
        return out;
    }
    if valid {
        out.stdout = vec![
            "valid: yes".into(),
            format!("value: {}", text(&result["value"])),
            format!(
                "fixes: {}",
                result["fixes"]
                    .as_array()
                    .map(|a| a.iter().map(text).collect::<Vec<_>>().join(", "))
                    .unwrap_or_default()
            ),
        ];
    } else {
        out.stderr = error_lines(&result, false);
    }
    out
}
fn detect(args: &Args, options: &PlatformOptions) -> Output {
    let input = match input::resolve(args.args.first().map(String::as_str), args.file.as_deref()) {
        Ok(Some(value)) => value,
        Ok(None) => return Output::usage("Missing value. Pass an argument or use --file."),
        Err(message) => return Output::usage(message),
    };
    let result = match br::detect(&input, options) {
        Ok(valid) => {
            let mut value = json!(valid);
            value["ok"] = true.into();
            value
        }
        Err(error) => {
            json!({"ok":false,"type":error.ty.map(|ty|ty.as_str()).unwrap_or("unknown"),"code":error.code,"message":error.message})
        }
    };
    let valid = result["ok"] == true;
    let mut out = render(&result, u8::from(!valid), args);
    if args.json || args.quiet {
        return out;
    }
    let mut lines = vec![format!("type: {}", text(&result["type"]))];
    if valid {
        lines.extend([
            "valid: yes".into(),
            format!("value: {}", text(&result["value"])),
        ]);
        if let Some(format) = result["format"].as_str() {
            lines.push(format!("format: {format}"));
        }
        if let Some(meta) = result.get("meta") {
            lines.push(format!("meta: {meta}"));
        }
        out.stdout = lines;
    } else {
        lines.extend(error_lines(&result, false));
        out.stderr = lines;
    }
    out
}
fn compare(command: &str, ty: DocumentType, args: &Args, opts: &PlatformOptions) -> Output {
    let a = args.args.get(1).map(String::as_str).unwrap_or_default();
    let b = args.args.get(2..).unwrap_or_default().join(" ");
    if a.is_empty() || b.is_empty() {
        return Output::usage(format!(
            "Missing values. Usage: {command} <type> <valueA> <valueB>"
        ));
    }
    if command == "compare" {
        let result = br::compare::compare_with_options(a, &b, ty, opts);
        let mut out = render(&json!(result), u8::from(!result.equal), args);
        if !args.json && !args.quiet {
            out.stdout.push(format!(
                "equal: {}",
                if result.equal { "yes" } else { "no" }
            ));
        }
        return out;
    }
    let result = br::diff::diff_with_options(a, &b, ty, opts);
    let mut out = render(&json!(result), u8::from(result.changed), args);
    if !args.json && !args.quiet {
        out.stdout.push(format!(
            "changed: {}",
            if result.changed { "yes" } else { "no" }
        ));
        for field in result.fields {
            out.stdout.extend([
                format!("field: {}", field.field),
                format!("  a: {}", field.before),
                format!("  b: {}", field.after),
            ]);
        }
    }
    out
}
fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_default()
}
fn render(value: &Value, code: u8, args: &Args) -> Output {
    if args.json {
        Output::json(value, code)
    } else {
        Output {
            exit_code: code,
            ..Output::default()
        }
    }
}
fn error_lines(value: &Value, format: bool) -> Vec<String> {
    let mut lines = Vec::new();
    if !format {
        lines.push("valid: no".into());
    }
    lines.extend([
        format!("code: {}", text(&value["code"])),
        format!("message: {}", text(&value["message"])),
    ]);
    lines
}
