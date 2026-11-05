//! Contrato das quatro tabelas do SPED.
use super::*;
use br::cst::*;
pub(super) fn run(action: &str, value: &str, args: &Args) -> Output {
    let Some(tax) = args
        .tax
        .as_deref()
        .and_then(|value| value.trim().parse::<CstTax>().ok())
    else {
        return Output::usage("Missing or invalid --tax. Expected: icms | ipi | pis | cofins");
    };
    if value.is_empty() {
        return Output::usage(format!(
            "Missing {}. Usage: br-validators cst {action} <{}> --tax icms",
            if action == "search" { "query" } else { "code" },
            if action == "search" {
                "query"
            } else {
                "codigo"
            }
        ));
    }
    match action {
        "lookup" => {
            let result = match tax {
                CstTax::Icms => lookup_cst_icms_por_codigo(value),
                CstTax::Ipi => lookup_cst_ipi_por_codigo(value),
                CstTax::Pis => lookup_cst_pis_por_codigo(value),
                CstTax::Cofins => lookup_cst_cofins_por_codigo(value),
            };
            match result {
                Ok(row) => {
                    let mut lines = vec![human("cst", &json!(row))];
                    if args.verbose {
                        lines.push(format!("tax: {tax}"));
                    }
                    success(
                        json!({"ok":true,"cst":row,"tax":tax}),
                        lines,
                        Some("cst"),
                        args,
                    )
                }
                Err(error) => lookup_error(error, 1, args),
            }
        }
        "search" => {
            let rows = match tax {
                CstTax::Icms => search_cst_icms(value, args.limit),
                CstTax::Ipi => search_cst_ipi(value, args.limit),
                CstTax::Pis => search_cst_pis(value, args.limit),
                CstTax::Cofins => search_cst_cofins(value, args.limit),
            };
            let lines = if rows.is_empty() {
                vec!["No matches.".into()]
            } else {
                rows.iter().map(|row| human("cst", &json!(row))).collect()
            };
            success(
                json!({"ok":true,"tax":tax,"results":rows}),
                lines,
                None,
                args,
            )
        }
        "validate" => match validate_cst(tax, value) {
            Ok(row) => {
                let mut lines = vec![format!("{} — {}", row.value, row.description)];
                if args.verbose {
                    lines.push(format!("tax: {tax}"));
                }
                success(
                    json!({"ok":true,"tax":tax,"value":row.value,"description":row.description}),
                    lines,
                    None,
                    args,
                )
            }
            Err(error) => lookup_error(error, 1, args),
        },
        _ => Output::usage(format!("Unknown command: cst {action}")),
    }
}
