//! Conversão decimal textual e cálculo exclusivamente inteiro de valores monetários.
use super::*;
fn cents(raw: &str) -> Option<i64> {
    let raw = raw.trim().replace(',', ".");
    let negative = raw.starts_with('-');
    let raw = raw.strip_prefix(['-', '+']).unwrap_or(&raw);
    let (whole, fraction) = raw.split_once('.').unwrap_or((raw, ""));
    if whole.is_empty() && fraction.is_empty()
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let whole = if whole.is_empty() {
        0
    } else {
        whole.parse::<i64>().ok()?
    };
    let mut decimals = fraction.bytes().chain(std::iter::repeat(b'0'));
    let part = i64::from(decimals.next()?.checked_sub(b'0')?) * 10
        + i64::from(decimals.next()?.checked_sub(b'0')?);
    let round = i64::from(decimals.next()? >= b'5');
    let value = whole
        .checked_mul(100)?
        .checked_add(part)?
        .checked_add(round)?;
    if negative {
        value.checked_neg()
    } else {
        Some(value)
    }
}
fn money(value: i64) -> String {
    format!(
        "{}{}.{:02}",
        if value < 0 { "-" } else { "" },
        value.unsigned_abs() / 100,
        value.unsigned_abs() % 100
    )
}
fn reais(value: i64) -> Value {
    serde_json::from_str(&money(value)).unwrap_or(Value::Null)
}
pub(super) fn run(command: &str, action: &str, value: &str, args: &Args) -> Output {
    let ano = args
        .ano
        .filter(|y| (1900..=2100).contains(y))
        .unwrap_or(2025);
    let label = if command == "irpf" {
        "IRPF progressive table"
    } else {
        "INSS contribution table"
    };
    if action == "tabela" {
        let (faixas, teto) = if command == "irpf" {
            let Some(rows) = br::irpf::get_irpf_tabela_progressiva(Some(ano)) else {
                return failure(format!("{label} not found for year {ano}"), 1);
            };
            (json!(rows), None)
        } else {
            let Some(table) = br::inss::get_inss_tabela_contribuicao(Some(ano)) else {
                return failure(format!("{label} not found for year {ano}"), 1);
            };
            (json!(table.faixas), Some(table.teto))
        };
        let mut payload = json!({"ok":true,"ano":ano,"faixas":faixas});
        let mut lines = vec![];
        if let Some(teto) = teto {
            payload["teto"] = json!(teto);
            lines.push(format!("Teto: R$ {teto:.2}"));
        }
        for row in faixas.as_array().into_iter().flatten() {
            let rate = row["aliquota"].as_f64().unwrap_or_default() * 100.;
            let mut line = format!(
                "Faixa {} — {} — {}%",
                field(row, "faixa"),
                field(row, "descricao"),
                rate.to_string().replace('.', ",")
            );
            if command == "irpf" {
                line.push_str(&format!(
                    " — deduzir R$ {:.2}",
                    row["parcelaDeduzir"].as_f64().unwrap_or_default()
                ));
            }
            lines.push(line);
        }
        return success(payload, lines, Some(command), args);
    }
    if action != "calc" {
        return Output::usage(format!("Unknown command: {command} {action}"));
    }
    let (name, usage) = if command == "irpf" {
        ("base de cálculo", "base")
    } else {
        ("salário de contribuição", "salario")
    };
    if value.is_empty() {
        return Output::usage(format!(
            "Missing {name}. Usage: br-validators {command} calc <{usage}>"
        ));
    }
    let Some(amount) = cents(value) else {
        return Output::usage(format!("Invalid {name}. Pass a numeric value (e.g. 3000)."));
    };
    let (mut payload, line) = if command == "irpf" {
        let Some(row) = br::irpf::calcular_irpf_mensal(amount, Some(ano)) else {
            return failure(
                if br::irpf::get_irpf_tabela_progressiva(Some(ano)).is_none() {
                    format!("{label} not found for year {ano}")
                } else {
                    "Base de cálculo must be a non-negative number".into()
                },
                1,
            );
        };
        let mut payload = json!(row);
        for (key, value) in [
            ("baseCalculo", row.base_calculo),
            ("parcelaDeduzir", row.parcela_deduzir),
            ("imposto", row.imposto),
        ] {
            payload[key] = reais(value);
        }
        (
            payload,
            format!(
                "IRPF {ano} — base R$ {} — faixa {} — imposto R$ {}",
                money(row.base_calculo),
                row.faixa,
                money(row.imposto)
            ),
        )
    } else {
        let Some(row) = br::inss::calcular_inss_mensal(amount, Some(ano)) else {
            return failure(
                if br::inss::get_inss_tabela_contribuicao(Some(ano)).is_none() {
                    format!("{label} not found for year {ano}")
                } else {
                    "Salário de contribuição must be a non-negative number".into()
                },
                1,
            );
        };
        let mut payload = json!(row);
        for (key, value) in [
            ("salarioContribuicao", row.salario_contribuicao),
            ("salarioEfetivo", row.salario_efetivo),
            ("contribuicao", row.contribuicao),
            ("teto", row.teto),
        ] {
            payload[key] = reais(value);
        }
        (
            payload,
            format!(
                "INSS {ano} — salário R$ {} — faixa {} — contribuição R$ {}",
                money(row.salario_contribuicao),
                row.faixa,
                money(row.contribuicao)
            ),
        )
    };
    payload["ok"] = true.into();
    success(payload, vec![line], Some(command), args)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decimal_exato() {
        assert_eq!(cents("3000,01"), Some(300001));
        assert_eq!(cents("0.005"), Some(1));
        assert_eq!(cents("-0.01"), Some(-1));
        assert_eq!(cents("NaN"), None);
        assert_eq!(cents("9999999999999999999999999999"), None);
    }
}
