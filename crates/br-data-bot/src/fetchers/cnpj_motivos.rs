use br_validators::schema::{Agendamento, MotivoSituacaoCadastral};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{rfb, DatasetFetcher};

const MIRROR: &str = "https://github.com/jonathands/dados-abertos-receita-cnpj/releases/download/2024.09/Motivos.zip";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "cnpj-motivos",
    nome: "RFB CNPJ Motivos de Situação Cadastral",
    fonte: "Receita Federal — Dados Abertos CNPJ (Motivos.zip)",
    endpoints: &[rfb::BASE, MIRROR],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#cnpj-motivos-situacao-cadastral",
    depends_on: &[],
    fetch,
};

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let csv = rfb::load_csv(ctx, "Motivos.zip", MIRROR, "moti")?;
    let records = rfb::parse_codes(&csv, 2, |codigo, descricao| MotivoSituacaoCadastral {
        codigo,
        descricao,
    })?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "motivos.json",
        records,
        Bounds::new(55, 65),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/cnpj-motivos/Motivos.zip");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/cnpj-motivos/motivos.json");

    #[test]
    fn interpreta_o_zip_e_respeita_o_diretorio_mensal_oficial() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let url = format!("{}2026-10/Motivos.zip", rfb::BASE);
        let http = FakeHttp::new(
            [
                (
                    rfb::BASE.into(),
                    Ok(b"<a href='2026-10/'>Outubro</a>".to_vec()),
                ),
                (url, Ok(FIXTURE.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, 61);
        let records: Vec<MotivoSituacaoCadastral> = serde_json::from_slice(&files[0].bytes)?;
        let seed: Vec<MotivoSituacaoCadastral> = serde_json::from_slice(SEED)?;
        for codigo in ["00", "01", "02"] {
            let found = records.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        Ok(())
    }

    #[test]
    fn usa_espelho_quando_a_receita_bloqueia_e_propaga_404() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let blocked = SourceError::Blocked("tempo esgotado".into());
        let http = FakeHttp::new(
            [
                (rfb::BASE.into(), Err(blocked.clone())),
                (MIRROR.into(), Ok(FIXTURE.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].count, 61);
        let http = FakeHttp::new(
            [
                (rfb::BASE.into(), Err(blocked)),
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
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
