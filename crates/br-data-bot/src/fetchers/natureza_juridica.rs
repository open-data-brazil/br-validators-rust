use br_validators::schema::{Agendamento, NaturezaJuridica};

use crate::framework::{
    context::FetchContext,
    csv,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{rfb, DatasetFetcher};

const MIRROR: &str = "https://github.com/jonathands/dados-abertos-receita-cnpj/releases/download/2024.09/Naturezas.zip";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "natureza-juridica",
    nome: "RFB CNPJ Naturezas Jurídicas",
    fonte: "Receita Federal — Dados Abertos CNPJ (Naturezas.zip)",
    endpoints: &[rfb::BASE, MIRROR],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#natureza-juridica",
    depends_on: &[],
    fetch,
};

fn parse(csv: &str) -> Result<Vec<NaturezaJuridica>, SourceError> {
    Ok(csv::rows(csv, b';')?
        .into_iter()
        .filter_map(|row| {
            let mut fields = row.into_iter();
            let digits: String = fields
                .next()?
                .chars()
                .filter(char::is_ascii_digit)
                .collect();
            let descricao = fields.next()?;
            if digits.is_empty() || descricao.is_empty() {
                return None;
            }
            let codigo = &digits[digits.len().saturating_sub(4)..];
            Some(NaturezaJuridica {
                codigo: format!("{codigo:0>4}"),
                descricao,
            })
        })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let csv = rfb::load_csv(ctx, "Naturezas.zip", MIRROR, "natju")?;
    let records = parse(&csv)?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "naturezas.json",
        records,
        Bounds::new(85, 95),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/natureza-juridica/Naturezas.zip");
    const SEED: &[u8] =
        include_bytes!("../../../br-validators/data/natureza-juridica/naturezas.json");

    #[test]
    fn prepara_o_zip_do_espelho_autorizado_e_preserva_a_tabela() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new(
            [
                (
                    rfb::BASE.into(),
                    Err(SourceError::Blocked("tempo esgotado".into())),
                ),
                (MIRROR.into(), Ok(FIXTURE.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, 90);
        let records: Vec<NaturezaJuridica> = serde_json::from_slice(&files[0].bytes)?;
        let seed: Vec<NaturezaJuridica> = serde_json::from_slice(SEED)?;
        for codigo in ["0000", "2062", "3271"] {
            let found = records.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        let http = FakeHttp::new(
            [
                (
                    rfb::BASE.into(),
                    Err(SourceError::Blocked("tempo esgotado".into())),
                ),
                (
                    MIRROR.into(),
                    Err(SourceError::LinkDeprecated {
                        url: MIRROR.into(),
                        status: 404,
                    }),
                ),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
