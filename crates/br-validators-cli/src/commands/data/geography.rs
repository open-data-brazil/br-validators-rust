//! Consultas geográficas, bancárias e de códigos eleitorais.
use super::*;

pub(super) fn run(command: &str, action: &str, value: &str, args: &Args) -> Output {
    if command == "ibge" && action == "list" {
        let target = if value.is_empty() {
            "municipios"
        } else {
            value
        };
        let mut rows = match target {
            "estados" => br::ibge::get_all_estados()
                .iter()
                .map(|row| json!(row))
                .collect(),
            "municipios" => br::ibge::get_all_municipios()
                .iter()
                .map(|row| json!(row))
                .collect(),
            _ => return Output::usage("Expected estados or municipios"),
        };
        filter_limit(&mut rows, args);
        let lines = rows
            .iter()
            .map(|row| {
                human(
                    if target == "estados" {
                        "estados"
                    } else {
                        "ibge"
                    },
                    row,
                )
            })
            .collect();
        let mut payload = json!({"ok":true,"total":rows.len(),target:rows});
        if target == "municipios" {
            if let Some(uf) = &args.uf {
                payload["uf"] = uf.trim().to_uppercase().into();
            }
        }
        return success(payload, lines, Some("ibge"), args);
    }
    if command == "bancos" && action == "list" {
        let mut rows = br::bancos::get_all_bancos()
            .iter()
            .map(|row| json!(row))
            .collect();
        filter_limit(&mut rows, args);
        return rows_output(command, "bancos", rows, None, args);
    }
    if action != "lookup" && !(command == "cep" && action == "faixa") {
        return Output::usage(format!("Unknown command: {command} {action}"));
    }
    if value.is_empty() {
        return Output::usage(match command {
            "ibge" => "Missing IBGE municipality code. Pass a 7-digit code.",
            "bancos" => "Missing bank code or ISPB. Pass COMPE (001) or ISPB (8 digits).",
            "ddd" => "Missing DDD code. Pass 2 digits (e.g. 11).",
            "cep" => "Missing CEP prefix. Pass at least 5 digits.",
            "tse-municipios" => {
                "Missing code. Pass a 5-digit TSE or 7-digit IBGE municipality code."
            }
            _ => "Missing cUF code. Usage: br-validators nfe-cuf lookup <code>",
        });
    }
    let digits = digits(value);
    match command {
        "bancos" => {
            let result = match digits.len() {
                1..=3 => br::bancos::lookup_banco_por_codigo(&digits),
                8 => br::bancos::lookup_banco_por_ispb(&digits),
                _ => return lookup_error(br::LookupError {code:br::LookupErrorCode::InvalidFormat,message:"Invalid bank code or ISPB. Use 3-digit COMPE (e.g. 001) or 8-digit ISPB.".into()},2,args),
            };
            match result {
                Ok(row) => single(command, "banco", json!(row), args),
                Err(error) => lookup_error(error, 1, args),
            }
        }
        "ibge" => {
            if digits.len() != 7 {
                return lookup_error(
                    br::LookupError {
                        code: br::LookupErrorCode::InvalidFormat,
                        message: "Invalid IBGE municipality code. Use 7 digits (e.g. 3550308)."
                            .into(),
                    },
                    2,
                    args,
                );
            }
            match br::ibge::lookup_municipio_por_codigo(digits.parse::<i64>().unwrap_or_default()) {
                Ok(row) => single(command, "municipio", json!(row), args),
                Err(error) => lookup_error(error, 1, args),
            }
        }
        "nfe-cuf" => match br::nfe_cuf::lookup_cuf_por_codigo(value) {
            Ok(row) => single(command, "cuf", json!(row), args),
            Err(error) => lookup_error(error, 1, args),
        },
        "cep" => {
            if digits.len() < 5 {
                return Output::usage("Invalid CEP prefix. Use at least 5 digits (e.g. 01310).");
            }
            match br::cep::get_cep_faixa_info(value) {
                Some(row) => single(command, "faixa", json!(row), args),
                None => failure(format!("CEP prefix not found: {value}"), 1),
            }
        }
        "ddd" => {
            if digits.is_empty() {
                return Output::usage("Invalid DDD. Use 2 digits (e.g. 11).");
            }
            let normalized = format!("{digits:0>2}");
            let ddd = normalized
                .get(normalized.len().saturating_sub(2)..)
                .unwrap_or_default();
            let Some(row) = br::telefone::get_ddd_info(ddd) else {
                return failure(format!("DDD not found: {value}"), 1);
            };
            let municipios = row
                .municipios
                .iter()
                .take(3)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            let suffix = if row.municipios.len() > 3 { "…" } else { "" };
            success(
                json!({"ok":true,"ddd":row}),
                vec![format!(
                    "DDD {} — {} / {} — {municipios}{suffix}",
                    row.ddd, row.uf, row.regiao
                )],
                Some(command),
                args,
            )
        }
        _ => tse(value, &digits, args),
    }
}
fn tse(value: &str, digits: &str, args: &Args) -> Output {
    let (payload, code, codes, is_tse) = match digits.len() {
        5 => {
            let Some(code) = br::tse_municipios::get_municipio_ibge_por_codigo_tse(digits) else {
                return failure(format!("Mapping not found: {value}"), 1);
            };
            (
                json!({"kind":"tse-to-ibge","codigoTse":digits,"ibgeCodigo":code}),
                code,
                vec![digits],
                true,
            )
        }
        7 => {
            let code = digits.parse::<u32>().unwrap_or_default();
            let codes = br::tse_municipios::get_codigos_tse_por_municipio(code);
            if codes.is_empty() {
                return failure(format!("Mapping not found: {value}"), 1);
            }
            (
                json!({"kind":"ibge-to-tse","ibgeCodigo":code,"codigosTse":codes}),
                code,
                codes.to_vec(),
                false,
            )
        }
        _ => {
            return Output::usage(
                "Invalid code. Use 5-digit TSE or 7-digit IBGE municipality code.",
            )
        }
    };
    let name = br::ibge::get_municipio_por_codigo(code)
        .map(|row| format!("{} ({})", row.nome, row.uf))
        .unwrap_or_else(|| code.to_string());
    let line = if is_tse {
        format!("TSE {digits} → IBGE {code} — {name}")
    } else {
        format!("IBGE {code} — {name} → TSE {}", codes.join(", "))
    };
    success(
        json!({"ok":true,"mapping":payload}),
        vec![line],
        Some("tse-municipios"),
        args,
    )
}
