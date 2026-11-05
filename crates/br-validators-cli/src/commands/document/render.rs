//! Mensagens humanas dos comandos de documentos.
use crate::{
    args::Args,
    output::{self, Output},
};
use serde_json::Value;
fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_default()
}
pub(super) fn validation(
    command: &str,
    action: &str,
    mut value: Value,
    args: &Args,
    source: Option<&str>,
) -> Output {
    let valid = value["ok"] == true;
    if valid {
        if let Some(source) = source {
            value["source"] = source.into();
        }
    }
    if args.json {
        return Output::json(&value, u8::from(!valid));
    }
    if args.quiet {
        return Output {
            exit_code: u8::from(!valid),
            ..Output::default()
        };
    }
    let mut lines = Vec::new();
    if command == "cnis" {
        lines.push(format!("ok: {valid}"));
        if valid {
            for key in ["value", "issuer", "tipo"] {
                lines.push(format!("{key}: {}", text(&value, key)));
            }
        } else {
            for key in ["code", "message"] {
                lines.push(format!("{key}: {}", text(&value, key)));
            }
        }
    } else if valid {
        match command {
            "brcode" => {
                lines.push("valid: yes".into());
                for key in [
                    "merchantName",
                    "merchantCity",
                    "amount",
                    "txid",
                    "pixKey",
                    "pixKeyType",
                    "pixInitiationUrl",
                ] {
                    if !text(&value, key).is_empty() {
                        lines.push(format!("{key}: {}", text(&value, key)));
                    }
                }
            }
            "telefone" | "pix" | "boleto" | "rg" | "ie" => {
                let key = match command {
                    "telefone" => "tipo",
                    "pix" => "keyType",
                    "boleto" => "inputKind",
                    _ => "uf",
                };
                lines.push(format!("valid: yes ({})", text(&value, key)));
                if command == "rg" {
                    lines.push(format!(
                        "checkDigitValidated: {}",
                        if value["checkDigitValidated"] == true {
                            "yes"
                        } else {
                            "no"
                        }
                    ));
                }
                if value["produtorRural"] == true {
                    lines.push("kind: produtor-rural".into());
                }
                lines.push(format!("value: {}", text(&value, "value")));
                lines.push(format!("format: {}", text(&value, "format")));
                if command == "boleto" {
                    for key in ["segment", "valueType", "situacao"] {
                        if !value[key].is_null() {
                            lines.push(format!("{key}: {}", text(&value, key)));
                        }
                    }
                }
            }
            "cartao" => {
                lines.push(format!("valid: yes ({})", text(&value, "format")));
                lines.push(format!("brand: {}", text(&value, "brand")));
                lines.push(format!("value: {}", text(&value, "value")));
            }
            _ => {
                let mut details = Vec::new();
                if action == "parse" && command == "nfe-chave" {
                    for key in ["cUF", "cnpj", "mod"] {
                        details.push(format!("{key}: {}", text(&value["parsed"], key)));
                    }
                    details.push(format!("uf: {}", text(&value, "uf")));
                }
                if action == "parse" && command == "processo-judicial" {
                    for key in [
                        "sequencial",
                        "checkDigits",
                        "ano",
                        "segmentoJustica",
                        "tribunal",
                        "origem",
                    ] {
                        details.push(format!("{key}: {}", text(&value["segments"], key)));
                    }
                }
                return output::validation(value, false, false, source, details);
            }
        }
    } else {
        lines.push("valid: no".into());
        for key in ["code", "message", "keyType", "inputKind", "uf", "brand"] {
            if !value[key].is_null() {
                lines.push(format!("{key}: {}", text(&value, key)));
            }
        }
        return Output {
            stderr: lines,
            exit_code: 1,
            ..Output::default()
        };
    }
    if valid {
        if let Some(source) = source {
            lines.push(format!("source: {source}"));
        }
    }
    Output {
        stdout: lines,
        exit_code: u8::from(!valid),
        ..Output::default()
    }
}
