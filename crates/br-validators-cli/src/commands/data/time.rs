//! Cotações e calendário; a data do relógio só é resolvida na execução.
use super::*;
use br::data::lookup::Data;

pub(super) fn run(command: &str, action: &str, value: &str, args: &Args) -> Output {
    match command {
        "feriados" if action == "list" => {
            let year = args
                .year
                .filter(|y| (1900..=2100).contains(y))
                .unwrap_or_else(|| Data::hoje_brasil().partes().0);
            let rows = br::feriados::get_all_feriados(year);
            let lines = rows
                .iter()
                .map(|row| {
                    format!(
                        "{} — {} ({})",
                        row.data,
                        row.nome,
                        field(&json!(row), "tipo")
                    )
                })
                .collect();
            success(
                json!({"ok":true,"year":year,"total":rows.len(),"feriados":rows}),
                lines,
                Some(command),
                args,
            )
        }
        "selic" => {
            if !action.is_empty() && action != "lookup" {
                return Output::usage(format!("Unknown command: selic {action}"));
            }
            let date = args
                .date
                .as_deref()
                .or(args.data.as_deref())
                .or((!value.is_empty()).then_some(value));
            let result = date.filter(|v| !v.trim().is_empty()).map_or_else(
                br::selic::get_selic_meta,
                br::selic::get_selic_meta_por_data,
            );
            let Some(row) = result else {
                return failure(
                    date.map(|date| format!("SELIC meta not found for date {date}"))
                        .unwrap_or_else(|| "SELIC meta series is empty".into()),
                    1,
                );
            };
            let mut lines = vec![format!(
                "SELIC meta {} — {}% a.a.",
                row.data_referencia, row.valor
            )];
            if args.verbose {
                lines.extend(stale_lines(row.data_referencia, row.is_stale, row.warning));
            }
            success(json!({"ok":true,"meta":row}), lines, Some(command), args)
        }
        "ptax" if action == "lookup" => {
            if value.is_empty() {
                return Output::usage("Missing currency code. Usage: ptax lookup <moeda> [data]");
            }
            let date = args
                .data
                .as_deref()
                .or(args.args.get(2).map(String::as_str));
            let Some(row) = br::ptax::get_ptax_cotacao(value, date) else {
                return failure(
                    format!(
                        "PTAX quote not found: {value}{}",
                        date.map(|date| format!(" {date}")).unwrap_or_default()
                    ),
                    1,
                );
            };
            let mut lines = vec![format!(
                "{} Fechamento PTAX — compra {} / venda {} ({})",
                row.moeda, row.cotacao_compra, row.cotacao_venda, row.data_referencia
            )];
            if args.verbose {
                lines.extend(stale_lines(row.data_referencia, row.is_stale, row.warning));
            }
            success(json!({"ok":true,"cotacao":row}), lines, Some(command), args)
        }
        "ptax" if action == "historico" => {
            if value.is_empty() {
                return Output::usage(
                    "Missing currency code. Usage: ptax historico <moeda> <desde> <ate>",
                );
            }
            let desde = args
                .desde
                .as_deref()
                .or(args.args.get(2).map(String::as_str))
                .unwrap_or_default();
            let ate = args
                .ate
                .as_deref()
                .or(args.args.get(3).map(String::as_str))
                .unwrap_or_default();
            if desde.trim().is_empty() || ate.trim().is_empty() {
                return Output::usage(
                    "Missing date range. Usage: ptax historico <moeda> <desde> <ate>",
                );
            }
            let rows = br::ptax::get_ptax_historico(value, desde, ate);
            if rows.is_empty() {
                return failure(
                    format!("No PTAX quotes in embed for {value} between {desde} and {ate}"),
                    1,
                );
            }
            let mut lines = vec![format!(
                "{} PTAX historico ({} rows)",
                value.to_uppercase(),
                rows.len()
            )];
            lines.extend(rows.iter().map(|row| {
                format!(
                    "{} — compra {} / venda {}",
                    row.data_referencia, row.cotacao_compra, row.cotacao_venda
                )
            }));
            let mut payload = json!({"ok":true,"moeda":value.to_uppercase(),"desde":desde,"ate":ate,"total":rows.len(),"cotacoes":rows});
            if args.verbose {
                payload["janelaDiasUteis"] = 90.into();
                lines.push("janelaDiasUteis: 90".into());
            }
            success(payload, lines, Some(command), args)
        }
        _ => Output::usage(format!("Unknown command: {command} {action}")),
    }
}
fn stale_lines(date: &str, stale: bool, warning: Option<&str>) -> Vec<String> {
    let mut lines = vec![
        format!("dataReferencia: {date}"),
        format!("isStale: {stale}"),
    ];
    if let Some(warning) = warning {
        lines.push(format!("warning: {warning}"));
    }
    lines
}
