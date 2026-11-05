//! Contratos JSON dos resultados especializados.
use crate::args::Args;
use br_validators::{
    self as br,
    types::{Uf, ValidationError, ValidationResult},
};
use serde_json::{json, Value};

pub(super) fn error(error: &ValidationError) -> Value {
    json!({"ok":false,"code":error.code.as_str(),"message":error.message})
}
pub(super) fn simple<T: AsRef<str>>(result: ValidationResult<T>) -> Value {
    match result {
        Ok(valid) => json!({"ok":true,"value":valid.value.as_ref(),"format":valid.format.as_str()}),
        Err(failure) => error(&failure),
    }
}
fn success(mut value: Value) -> Value {
    value["ok"] = true.into();
    value
}
pub(super) fn boleto(result: br::boleto::BoletoValidationResult) -> Value {
    match result {
        Ok(valid) => success(json!(valid)),
        Err(failure) => {
            let mut value = error(&failure.error);
            if let Some(kind) = failure.input_kind {
                value["inputKind"] = kind.as_str().into();
            }
            value
        }
    }
}
pub(super) fn specialized(
    command: &str,
    action: &str,
    input: &str,
    args: &Args,
    uf: Option<Uf>,
    rural: bool,
) -> Result<Value, String> {
    Ok(match command {
        "telefone" => match br::validate_telefone(input) {
            Ok(v) => success(json!(v)),
            Err(e) => error(&e),
        },
        "cnis" => match br::validate_nit(input) {
            Ok(v) => {
                let mut value = success(json!(v));
                if let Some(issuer) = &args.issuer {
                    value["issuer"] = issuer.clone().into();
                }
                if let Some(tipo) = &args.tipo {
                    value["tipo"] = tipo.clone().into();
                }
                value
            }
            Err(e) => error(&e),
        },
        "cartao" => match br::validate_cartao_credito(input) {
            Ok(v) => success(json!(v)),
            Err(e) => error(&e),
        },
        "pix" => {
            let kind = args
                .pix_type
                .as_deref()
                .map(|v| {
                    serde_json::from_value::<br::pix::PixKeyType>(json!(v))
                        .map_err(|_| format!("Invalid PIX key type: {v}"))
                })
                .transpose()?;
            match br::pix::validate_pix_key_with_options(
                input,
                br::pix::ValidatePixKeyOptions { key_type: kind },
            ) {
                Ok(v) => success(json!(v)),
                Err(e) => {
                    let mut v = error(&e.error);
                    if let Some(kind) = e.key_type {
                        v["keyType"] = kind.as_str().into();
                    }
                    v
                }
            }
        }
        "boleto" => {
            let kind = args
                .kind
                .as_deref()
                .map(|v| {
                    serde_json::from_value::<br::boleto::BoletoInputKind>(json!(v))
                        .map_err(|_| format!("Invalid boleto input kind: {v}"))
                })
                .transpose()?;
            boleto(br::validate_boleto_with_options(
                input,
                br::boleto::ValidateBoletoOptions {
                    kind,
                    ..Default::default()
                },
            ))
        }
        "brcode" => match br::parse_brcode(input) {
            Ok(v) => success(json!(v)),
            Err(e) => error(&e),
        },
        "rg" => {
            let uf = uf.ok_or("Missing --uf")?;
            match br::validate_rg(input, uf) {
                Ok(v) => success(json!(v)),
                Err(e) => {
                    let mut v = error(&e.error);
                    v["uf"] = json!(e.uf);
                    v
                }
            }
        }
        "ie" => {
            let uf = uf.ok_or("Missing --uf")?;
            if rural {
                match br::validate_ie_sp_rural(input) {
                    Ok(v) => {
                        let mut v = success(json!(v));
                        v["produtorRural"] = true.into();
                        v
                    }
                    Err(e) => {
                        let mut v = error(&e.error);
                        if let Some(uf) = e.uf {
                            v["uf"] = json!(uf);
                        }
                        v
                    }
                }
            } else {
                match br::validate_inscricao_estadual(input, uf) {
                    Ok(v) => success(json!(v)),
                    Err(e) => {
                        let mut v = error(&e.error);
                        if let Some(uf) = e.uf {
                            v["uf"] = json!(uf);
                        }
                        v
                    }
                }
            }
        }
        "nfe-chave" if action == "parse" => match br::validate_nfe_chave(input) {
            Ok(v) => success(json!(v)),
            Err(e) => error(&e),
        },
        "processo-judicial" if action == "parse" => match br::validate_processo_judicial(input) {
            Ok(v) => success(json!(v)),
            Err(e) => error(&e),
        },
        _ => {
            let ty = command
                .parse::<br::platform::DocumentType>()
                .map_err(|e| e.to_string())?;
            simple(br::platform::validate_document(
                input,
                ty,
                &br::platform::PlatformOptions { uf },
            ))
        }
    })
}
