use br_validators::schema::{Agendamento, Csosn};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{static_source::probe, DatasetFetcher};

const URL: &str = "https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/a03_10";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "csosn",
    nome: "CONFAZ CSOSN — Simples Nacional",
    fonte: "CONFAZ Ajuste SINIEF 03/10 — CSOSN table",
    endpoints: &[URL],
    agendamento: Agendamento::Estatico,
    documentacao: "docs/OFFICIAL-SOURCES.md#csosn",
    depends_on: &[],
    fetch,
};

// Fonte: https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/a03_10
const CODIGOS: &[(&str, &str)] = &[
    ("101", "Tributada pelo Simples Nacional com permissão de crédito"),
    ("102", "Tributada pelo Simples Nacional sem permissão de crédito"),
    ("103", "Isenção do ICMS no Simples Nacional para faixa de receita bruta"),
    ("201", "Tributada pelo Simples Nacional com permissão de crédito e com cobrança do ICMS por substituição tributária"),
    ("202", "Tributada pelo Simples Nacional sem permissão de crédito e com cobrança do ICMS por substituição tributária"),
    ("203", "Isenção do ICMS no Simples Nacional para faixa de receita bruta e com cobrança do ICMS por substituição tributária"),
    ("300", "Imune"),
    ("400", "Não tributada pelo Simples Nacional"),
    ("500", "ICMS cobrado anteriormente por substituição tributária (substituído) ou por antecipação"),
    ("900", "Outros"),
];

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    probe(ctx, URL)?;
    let records = CODIGOS
        .iter()
        .map(|&(codigo, descricao)| Csosn {
            codigo: codigo.into(),
            descricao: descricao.into(),
        })
        .collect();
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "csosn.json",
        records,
        Bounds::new(10, 10),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const SEED: &[u8] = include_bytes!("../../../br-validators/data/csosn/csosn.json");

    #[test]
    fn gera_os_dez_codigos_do_ajuste_sinief() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 27).expect("data da semente");
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
        assert_eq!(
            fetch(&ctx).expect_err("probe falhou").http_status(),
            Some(404)
        );
        Ok(())
    }
}
