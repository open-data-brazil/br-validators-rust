//! Apresentação do ISS com o aviso obrigatório da referência.
use super::*;
use br::iss_municipal::*;
pub(super) fn run(action: &str, value: &str, args: &Args) -> Output {
    let mut out = match action {
        "lookup" => {
            if value.is_empty() {
                return Output::usage(
                    "Missing IBGE code. Usage: br-validators iss-municipal lookup <codigoIbge>",
                );
            }
            let Some(row) = lookup_iss_municipal_por_ibge(value) else {
                return failure(
                    format!("ISS municipal row not found for IBGE code {value}"),
                    1,
                );
            };
            let mut lines = vec![
                human("iss-municipal", &json!(row)),
                format!("fonte: {}", field(&json!(row), "fonte")),
                format!("warning: {}", row.warning),
            ];
            if args.verbose {
                lines.extend([
                    format!("leiUrl: {}", row.lei_url),
                    format!("estimativa: {}", row.estimativa),
                ]);
            }
            success(
                json!({"ok":true,"iss":row}),
                lines,
                Some("iss-municipal"),
                args,
            )
        }
        "resolve" => {
            let nome = args.args.get(2..).unwrap_or_default().join(" ");
            if value.is_empty() || nome.trim().is_empty() {
                return Output::usage("Missing UF or municipality name. Usage: br-validators iss-municipal resolve <uf> <nome>");
            }
            let Some(row) = get_iss_municipal_por_uf_municipio(value, &nome) else {
                return failure(format!("ISS municipal row not found for {nome}/{value}"), 1);
            };
            return run("lookup", &row.codigo_ibge.to_string(), args);
        }
        "list" => {
            let Some(uf) = args
                .uf
                .as_deref()
                .map(str::trim)
                .filter(|uf| !uf.is_empty())
            else {
                return Output::usage(
                    "Missing UF. Usage: br-validators iss-municipal list --uf <UF>",
                );
            };
            let mut rows = get_iss_municipal_por_uf(uf);
            if let Some(limit) = args.limit.filter(|&v| v > 0) {
                rows.truncate(limit);
            }
            let lines = if rows.is_empty() {
                vec![format!(
                    "No ISS municipal rows embedded for UF {}.",
                    uf.to_uppercase()
                )]
            } else {
                rows.iter()
                    .map(|row| human("iss-municipal", &json!(row)))
                    .collect()
            };
            success(
                json!({"ok":true,"uf":uf.to_uppercase(),"total":rows.len(),"results":rows}),
                lines,
                Some("iss-municipal"),
                args,
            )
        }
        "search" => {
            if value.is_empty() {
                return Output::usage(
                    "Missing query. Usage: br-validators iss-municipal search <query>",
                );
            }
            let rows = search_iss_municipal(value, args.uf.as_deref(), args.limit);
            let lines = if rows.is_empty() {
                vec!["No ISS municipal rows matched.".into()]
            } else {
                rows.iter()
                    .map(|row| human("iss-municipal", &json!(row)))
                    .collect()
            };
            success(json!({"ok":true,"results":rows}), lines, None, args)
        }
        _ => return Output::usage(format!("Unknown command: iss-municipal {action}")),
    };
    if !args.json {
        out.stderr.push(ISS_MUNICIPAL_ESTIMATION_WARNING.into());
    }
    out
}
