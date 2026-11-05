//! Comandos dos documentos, delegando os algoritmos à biblioteca.
mod contracts;
mod render;
mod sources;
use crate::{
    args::Args,
    input,
    output::{self, Output},
};
use br_validators::{self as br, platform::DocumentType, types::Uf};
use serde_json::{json, Value};

fn canonical(command: &str) -> &str {
    match command {
        "titulo" => "titulo-eleitor",
        "nfe" => "nfe-chave",
        "cartao-credito" => "cartao",
        _ => command,
    }
}
/// Identifica os comandos de documentos e seus aliases.
pub fn recognizes(command: &str) -> bool {
    matches!(
        canonical(command),
        "cpf"
            | "cnpj"
            | "cep"
            | "telefone"
            | "cnh"
            | "renavam"
            | "titulo-eleitor"
            | "processo-judicial"
            | "rg"
            | "nfe-chave"
            | "placa"
            | "pis-pasep"
            | "cnis"
            | "pix"
            | "brcode"
            | "boleto"
            | "cartao"
            | "ean"
            | "ie"
    )
}
fn document_type(command: &str, rural: bool) -> Result<DocumentType, String> {
    match command {
        "cartao" => Ok(DocumentType::CartaoCredito),
        "ie" => Ok(if rural {
            DocumentType::InscricaoEstadualProdutorRural
        } else {
            DocumentType::InscricaoEstadual
        }),
        _ => command
            .parse()
            .map_err(|e: br::platform::ParseDocumentTypeError| e.to_string()),
    }
}
fn actions(command: &str) -> &'static [&'static str] {
    match command {
        "brcode" => &["parse", "validate"],
        "pix" => &["detect", "validate", "format"],
        "boleto" => &["detect", "validate", "format", "strip", "convert"],
        "nfe-chave" | "processo-judicial" => &["validate", "parse", "format", "strip"],
        "placa" => &["validate", "format", "strip", "convert"],
        "cartao" | "ean" => &["detect", "validate", "format", "strip"],
        _ => &["validate", "format", "strip"],
    }
}
fn label(command: &str) -> &str {
    match command {
        "cpf" => "CPF",
        "cnpj" => "CNPJ",
        "cep" => "CEP",
        "telefone" => "telephone",
        "cnh" => "CNH",
        "renavam" => "RENAVAM",
        "titulo-eleitor" => "Título de Eleitor",
        "processo-judicial" => "processo judicial",
        "rg" => "RG",
        "nfe-chave" => "NF-e chave de acesso",
        "placa" => "placa",
        "pis-pasep" => "PIS/PASEP",
        "cnis" => "NIT",
        "pix" => "PIX key",
        "boleto" => "boleto",
        "cartao" => "credit card PAN",
        "ean" => "EAN",
        "ie" => "IE",
        _ => "",
    }
}
fn format_contract(result: br::types::FormatResult) -> Value {
    match result {
        Ok(text) => json!({"ok":true,"formatted":text}),
        Err(error) => contracts::error(&error),
    }
}
fn detect(command: &str, input: &str, args: &Args) -> Output {
    let (key, value) = match command {
        "pix" => (
            "keyType",
            br::detect_pix_key_type(input).map_or("unknown", |v| v.as_str()),
        ),
        "ean" => (
            "format",
            br::ean::detect_ean_format(input).map_or("unknown", |v| v.as_str()),
        ),
        "cartao" => (
            "brand",
            br::detect_card_brand(&br::strip_cartao_credito(input)).as_str(),
        ),
        _ => (
            "inputKind",
            br::detect_boleto_input_kind(input).map_or("unknown", |v| v.as_str()),
        ),
    };
    if args.json {
        Output::json(&json!({key:value}), 0)
    } else if args.quiet {
        Output::default()
    } else {
        Output::text(value)
    }
}
fn strip(command: &str, input: &str, uf: Option<Uf>, rural: bool) -> String {
    match command {
        "cnpj" => br::strip_cnpj(input),
        "telefone" => br::strip_telefone(input),
        "placa" => br::strip_placa(input),
        "rg" => uf.map_or_else(String::new, |uf| br::strip_rg(input, uf)),
        "ie" if rural => br::strip_ie_sp_rural(input),
        _ => br::util::chars::only_digits(input),
    }
}
/// Executa uma ação de documento e preserva seus fluxos e códigos de saída.
pub fn run(command: &str, args: &Args) -> Output {
    execute(canonical(command), args).unwrap_or_else(Output::usage)
}
fn execute(command: &str, args: &Args) -> Result<Output, String> {
    let action = args.args.first().map_or("", String::as_str);
    if !actions(command).contains(&action) {
        return Ok(Output::usage(format!(
            "Expected action: {}",
            actions(command).join(" | ")
        )));
    }
    let mut start = 1;
    let direction = if command == "boleto" && action == "convert" {
        start = 2;
        let direction = args.args.get(1).map_or("", String::as_str);
        if !matches!(direction, "linha-to-barras" | "barras-to-linha") {
            return Ok(Output::usage(
                "Expected: boleto convert linha-to-barras|barras-to-linha <value>",
            ));
        }
        Some(direction)
    } else {
        None
    };
    let values = args.args.get(start..).unwrap_or_default();
    let value = (!values.is_empty()).then(|| values.join(" "));
    let Some(input) = input::resolve(value.as_deref(), args.file.as_deref())? else {
        return Ok(Output::usage(if command == "brcode" {
            "Missing BR Code payload. Pass an argument or use --file.".to_owned()
        } else {
            format!(
                "Missing {} value. Pass an argument or use --file.",
                label(command)
            )
        }));
    };
    let uf = if matches!(command, "rg" | "ie") {
        let uf = args.uf.as_deref().and_then(|value| {
            let upper = value.to_uppercase();
            Uf::ALL.into_iter().find(|uf| uf.as_str() == upper)
        });
        if uf.is_none() {
            return Ok(Output::usage(format!(
                "Missing or invalid --uf. Use one of: {}.",
                if command == "rg" {
                    sources::RG_UFS
                } else {
                    sources::IE_UFS
                }
            )));
        }
        uf
    } else {
        None
    };
    let rural = command == "ie"
        && uf == Some(Uf::SP)
        && br::inscricao_estadual::sp_rural::is_sp_rural_ie_input(&input);
    let source = if args.source {
        Some(match command {
            "rg" => br::rg::get_rg_official_source_url(uf.ok_or("Missing --uf")?),
            "ie" if rural => {
                br::inscricao_estadual::sp_rural::get_ie_produtor_rural_official_source_url()
            }
            "ie" => br::inscricao_estadual::get_ie_official_source_url(uf.ok_or("Missing --uf")?),
            _ => sources::source(command),
        })
    } else {
        None
    };
    Ok(match action {
        "detect" => detect(command, &input, args),
        "strip" if command == "boleto" => {
            let kind = br::detect_boleto_input_kind(&input).map_or("unknown", |v| v.as_str());
            let stripped = br::util::chars::only_digits(&input);
            if args.json {
                Output::json(&json!({"stripped":stripped,"inputKind":kind}), 0)
            } else if args.quiet {
                Output::default()
            } else {
                Output::text(stripped)
            }
        }
        "strip" => output::strip(&strip(command, &input, uf, rural), args.json),
        "format" => {
            let result = if command == "cnis" {
                br::format_nit(&input)
            } else if command == "pix" {
                let kind = args
                    .pix_type
                    .as_deref()
                    .map(|v| {
                        serde_json::from_value(json!(v))
                            .map_err(|_| format!("Invalid PIX key type: {v}"))
                    })
                    .transpose()?;
                br::pix::format_pix_key_with_options(
                    &input,
                    br::pix::ValidatePixKeyOptions { key_type: kind },
                )
            } else {
                br::mask::mask(
                    &input,
                    document_type(command, rural)?,
                    &br::mask::MaskOptions { uf },
                )
            };
            output::format(format_contract(result), args.json, args.quiet)
        }
        "convert" if command == "placa" => output::format(
            format_contract(br::convert_placa_to_mercosul(&input)),
            args.json,
            args.quiet,
        ),
        "convert" => {
            let result = if direction == Some("linha-to-barras") {
                contracts::boleto(
                    br::boleto::convert_linha_to_codigo_barras(&input)
                        .map(br::boleto::ValidBoleto::CodigoBarras),
                )
            } else {
                contracts::boleto(
                    br::boleto::convert_codigo_barras_to_linha_digitavel(&input)
                        .map(br::boleto::ValidBoleto::LinhaDigitavel),
                )
            };
            let mut result = result;
            if let Some(map) = result.as_object_mut() {
                map.remove("situacao");
            }
            if args.json {
                Output::json(&result, u8::from(result["ok"] != true))
            } else if args.quiet {
                Output {
                    exit_code: u8::from(result["ok"] != true),
                    ..Output::default()
                }
            } else if result["ok"] == true {
                Output::text(result["value"].as_str().unwrap_or_default())
            } else {
                let mut out = output::format(result, false, false);
                out.stderr.retain(|line| !line.starts_with("inputKind:"));
                out
            }
        }
        _ => render::validation(
            command,
            action,
            contracts::specialized(command, action, &input, args, uf, rural)?,
            args,
            source,
        ),
    })
}
