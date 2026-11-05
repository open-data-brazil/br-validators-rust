//! Consultas offline da CLI; somente a apresentação conhece o contrato TypeScript.
mod fiscal;
mod geography;
mod reference;
mod time;
use crate::{args::Args, input::trim_input, output::Output};
use br_validators as br;
use serde_json::{json, Value};

const REFERENCES: &[&str] = &[
    "natureza-juridica",
    "nbs",
    "cest",
    "cnae",
    "cfop",
    "csosn",
    "ncm",
    "cbo",
    "moedas",
    "paises-bacen",
    "incoterms",
    "portos",
    "aeroportos",
];
pub fn recognizes(command: &str, args: &Args) -> bool {
    REFERENCES.contains(&command)
        || matches!(
            command,
            "data"
                | "ibge"
                | "bancos"
                | "ddd"
                | "tse-municipios"
                | "nfe-cuf"
                | "ptax"
                | "selic"
                | "feriados"
                | "irpf"
                | "inss"
                | "cst"
                | "iss-municipal"
        )
        || command == "cep" && args.args.first().is_some_and(|action| action == "faixa")
}
pub fn run(command: &str, args: &Args) -> Output {
    let action = args.args.first().map(String::as_str).unwrap_or_default();
    let value = args
        .args
        .get(1)
        .map(|value| trim_input(value))
        .unwrap_or_default();
    match command {
        "data" if action == "catalog" => {
            let rows = br::get_data_catalog();
            success(
                json!({"ok":true,"total":rows.len(),"datasets":rows}),
                rows.iter()
                    .map(|row| {
                        format!(
                            "{} — {} — capturadoEm: {}",
                            row.id, row.nome, row.capturado_em
                        )
                    })
                    .collect(),
                None,
                args,
            )
        }
        "ibge" | "bancos" | "ddd" | "cep" | "tse-municipios" | "nfe-cuf" => {
            geography::run(command, action, value, args)
        }
        "ptax" | "selic" | "feriados" => time::run(command, action, value, args),
        "irpf" | "inss" | "cst" | "iss-municipal" => fiscal::run(command, action, value, args),
        command if REFERENCES.contains(&command) => run_reference(command, action, value, args),
        _ => Output::usage(format!("Unknown command: {command} {action}")),
    }
}
fn run_reference(command: &str, action: &str, value: &str, args: &Args) -> Output {
    if action == "list" {
        let mut rows = reference::list(command);
        filter_limit(&mut rows, args);
        return rows_output(
            command,
            &format!("{}s", reference::key(command)),
            rows,
            None,
            args,
        );
    }
    if value.is_empty() {
        let missing = if action == "search" { "query" } else { "code" };
        return Output::usage(format!(
            "Missing {missing}. Usage: br-validators {command} {action} <{missing}>"
        ));
    }
    match action {
        "lookup" => match reference::lookup(command, value) {
            Ok(row) => single(command, reference::key(command), row, args),
            Err(error) => lookup_error(error, 1, args),
        },
        "search" if ["cnae", "cfop", "csosn", "ncm", "cbo"].contains(&command) => {
            let rows =
                reference::search(command, value, args.limit.filter(|&v| v > 0).or(Some(10)));
            if rows.is_empty() && !args.json {
                return failure(format!("No matches for: {value}"), 1);
            }
            rows_output(
                command,
                &format!("{}s", reference::key(command)),
                rows,
                Some(value),
                args,
            )
        }
        "validate" if matches!(command, "ncm" | "cfop" | "csosn") => {
            let result = match command {
                "ncm" => br::ncm::validate_ncm(value),
                "cfop" => br::cfop::validate_cfop(value),
                _ => br::csosn::validate_csosn(value),
            };
            match result {
                Ok(valid) => {
                    let mut payload = json!(valid);
                    payload["ok"] = true.into();
                    if args.verbose {
                        payload["module"] = command.into();
                    }
                    let mut lines = vec![format!("{} — {}", valid.value, valid.description)];
                    if let Some(format) = valid.format {
                        lines.push(format!("format: {format}"));
                    }
                    success(payload, lines, None, args)
                }
                Err(error) => lookup_error(error, 1, args),
            }
        }
        _ => Output::usage(format!("Unknown command: {command} {action}")),
    }
}
fn text(value: &Value) -> String {
    value.as_str().map_or_else(
        || {
            if value.is_null() {
                "—".into()
            } else {
                value.to_string()
            }
        },
        str::to_owned,
    )
}
fn field(row: &Value, key: &str) -> String {
    text(&row[key])
}
fn human(command: &str, row: &Value) -> String {
    match command {
        "bancos" => format!(
            "{} — {} (ISPB {})",
            field(row, "codigo"),
            field(row, "nome"),
            field(row, "ispb")
        ),
        "ibge" | "portos" => format!(
            "{} — {} ({})",
            field(row, "codigo"),
            field(row, "nome"),
            field(row, "uf")
        ),
        "estados" => format!(
            "{} — {} — {}",
            field(row, "codigo"),
            field(row, "sigla"),
            field(row, "nome")
        ),
        "moedas" => format!(
            "{} — {} ({})",
            field(row, "codigo"),
            field(row, "nome"),
            field(row, "simbolo")
        ),
        "incoterms" => format!(
            "{} — {} ({})",
            field(row, "codigo"),
            field(row, "nome"),
            field(row, "edicao")
        ),
        "paises-bacen" => format!("{} — {}", field(row, "codigo"), field(row, "nome")),
        "aeroportos" => format!(
            "{}/{} — {} ({})",
            field(row, "iata"),
            field(row, "icao"),
            field(row, "nome"),
            field(row, "uf")
        ),
        "nfe-cuf" => format!(
            "cUF {} — {} — {} (IBGE UF {})",
            field(row, "codigo"),
            field(row, "uf"),
            field(row, "nome"),
            field(row, "codigoIbge")
        ),
        "cep" => format!(
            "{} — {} ({}) · IBGE {}",
            field(row, "prefixo"),
            field(row, "cidade"),
            field(row, "uf"),
            field(row, "codigoIbge")
        ),
        "iss-municipal" => format!(
            "{}/{} — ISS {}%–{}%",
            field(row, "nome"),
            field(row, "uf"),
            field(row, "aliquotaMin"),
            field(row, "aliquotaMax")
        ),
        _ => format!("{} — {}", field(row, "codigo"), field(row, "descricao")),
    }
}
fn filter_limit(rows: &mut Vec<Value>, args: &Args) {
    if let Some(uf) = args.uf.as_ref() {
        rows.retain(|row| {
            row["uf"]
                .as_str()
                .is_some_and(|value| value.eq_ignore_ascii_case(uf.trim()))
        });
    }
    if let Some(limit) = args.limit.filter(|&v| v > 0) {
        rows.truncate(limit);
    }
}
fn rows_output(
    command: &str,
    key: &str,
    rows: Vec<Value>,
    query: Option<&str>,
    args: &Args,
) -> Output {
    let lines = rows.iter().map(|row| human(command, row)).collect();
    let mut payload = json!({"ok":true,"total":rows.len(),key:rows});
    if let Some(query) = query {
        payload["query"] = query.into();
    }
    success(payload, lines, Some(command), args)
}
fn single(command: &str, key: &str, row: Value, args: &Args) -> Output {
    let line = human(command, &row);
    success(json!({"ok":true,key:row}), vec![line], Some(command), args)
}
fn success(
    mut payload: Value,
    mut lines: Vec<String>,
    dataset: Option<&str>,
    args: &Args,
) -> Output {
    let dataset = match dataset {
        Some("cnae") => Some("cnaes"),
        Some("cep") => Some("cep-faixas"),
        Some("ddd") => Some("telefone-ddd"),
        value => value,
    };
    if args.verbose {
        if let Some(metadata) = dataset.and_then(br::get_dataset_metadata) {
            payload["capturadoEm"] = metadata.capturado_em.clone().into();
            lines.push(format!("capturadoEm: {}", metadata.capturado_em));
        }
    }
    if args.json {
        Output::json(&payload, 0)
    } else {
        Output {
            stdout: if args.quiet { vec![] } else { lines },
            ..Output::default()
        }
    }
}
fn lookup_error(error: br::LookupError, code: u8, args: &Args) -> Output {
    if args.json {
        Output::json(
            &json!({"ok":false,"code":error.code,"message":error.message}),
            code,
        )
    } else {
        failure(error.message.to_string(), code)
    }
}
fn failure(message: impl Into<String>, code: u8) -> Output {
    Output {
        stderr: vec![message.into()],
        exit_code: code,
        ..Output::default()
    }
}
fn digits(value: &str) -> String {
    value.chars().filter(char::is_ascii_digit).collect()
}
