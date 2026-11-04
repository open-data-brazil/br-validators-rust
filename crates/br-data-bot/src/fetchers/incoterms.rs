use br_validators::schema::{Agendamento, Incoterm};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{static_source::probe, DatasetFetcher};

const URL: &str = "https://iccwbo.org/resources-for-business/incoterms-rules/";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "incoterms",
    nome: "ICC Incoterms 2020",
    fonte: "International Chamber of Commerce — Incoterms 2020 (static reference)",
    endpoints: &[URL],
    agendamento: Agendamento::Estatico,
    documentacao: "docs/OFFICIAL-SOURCES.md#incoterms",
    depends_on: &[],
    fetch,
};

const INCOTERMS: &[(&str, &str)] = &[
    ("EXW", "Ex Works"),
    ("FCA", "Free Carrier"),
    ("CPT", "Carriage Paid To"),
    ("CIP", "Carriage and Insurance Paid To"),
    ("DAP", "Delivered at Place"),
    ("DPU", "Delivered at Place Unloaded"),
    ("DDP", "Delivered Duty Paid"),
    ("FAS", "Free Alongside Ship"),
    ("FOB", "Free On Board"),
    ("CFR", "Cost and Freight"),
    ("CIF", "Cost, Insurance and Freight"),
];

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    probe(ctx, URL)?;
    let records = INCOTERMS
        .iter()
        .map(|&(codigo, nome)| Incoterm {
            codigo: codigo.into(),
            nome: nome.into(),
            edicao: "2020".into(),
        })
        .collect();
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "incoterms.json",
        records,
        Bounds::new(11, 11),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::{http::FakeHttp, outcome::SourceFetchStatus};
    use chrono::NaiveDate;

    const SEED: &[u8] = include_bytes!("../../../br-validators/data/incoterms/incoterms.json");

    #[test]
    fn gera_as_regras_da_icc_e_preserva_os_dados_quando_o_probe_falha() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da semente");
        let http = FakeHttp::new([(URL.into(), Ok(Vec::new()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].bytes, SEED);

        let http = FakeHttp::new(
            [(
                URL.into(),
                Err(SourceError::LinkDeprecated {
                    url: URL.into(),
                    status: 404,
                }),
            )]
            .into(),
        );
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let error = fetch(&ctx).expect_err("probe falhou");
        assert_eq!(error.status(), SourceFetchStatus::EmbeddedRetained);
        assert_eq!(error.http_status(), Some(404));
        Ok(())
    }
}
