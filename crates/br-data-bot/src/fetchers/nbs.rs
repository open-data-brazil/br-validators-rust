use std::collections::BTreeMap;

use br_validators::schema::{Agendamento, Nbs};

use crate::framework::{
    context::FetchContext,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::fold,
    xlsx,
};

use super::DatasetFetcher;

const URL: &str = "https://www.gov.br/nfse/pt-br/biblioteca/documentacao-tecnica/documentacao-atual/anexo_b-nbs2-lista_servico_nacional-snnfse.xlsx/@@download/file/ANEXO_B-NBS2-LISTA_SERVICO_NACIONAL-SNNFSe.xlsx";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "nbs",
    nome: "NFSe NBS",
    fonte: "NFSe Nacional — Anexo B NBS2 Lista Serviço Nacional",
    endpoints: &[URL],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#nbs",
    depends_on: &[],
    fetch,
};

fn leaf(code: &str) -> bool {
    let mut groups = code.split('.');
    [1, 4, 2, 2].into_iter().all(|len| {
        groups
            .next()
            .is_some_and(|part| part.len() == len && part.bytes().all(|b| b.is_ascii_digit()))
    }) && groups.next().is_none()
}

fn parse(bytes: &[u8]) -> Result<Vec<Nbs>, SourceError> {
    let rows = xlsx::read_sheet(bytes, Some("LISTA.NBS_v2.0"))?;
    let mut rows = rows.into_iter();
    let (codigo, descricao) = rows
        .find_map(|row| {
            let cols: Vec<_> = row.iter().map(|s| fold(s)).collect();
            let codigo = cols.iter().position(|s| s == "CODIGO NBS")?;
            let descricao = cols.iter().position(|s| s == "DESCRICAO")?;
            Some((codigo, descricao))
        })
        .ok_or_else(|| SourceError::Parse("cabeçalho Código NBS / Descrição ausente".into()))?;

    let mut records = BTreeMap::new();
    for row in rows {
        if let (Some(codigo), Some(descricao)) = (row.get(codigo), row.get(descricao)) {
            if leaf(codigo) && !descricao.is_empty() {
                records
                    .entry(codigo.clone())
                    .or_insert_with(|| descricao.clone());
            }
        }
    }
    Ok(records
        .into_iter()
        .map(|(codigo, descricao)| Nbs { codigo, descricao })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let response = ctx.http.get(URL, &RequestOpts::default())?;
    let records = parse(&response.bytes)?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "nbs.json",
        records,
        Bounds::new(800, 950),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/nbs/anexo-b.xlsx");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/nbs/nbs.json");

    #[test]
    fn identifica_o_cabecalho_e_os_codigos_folha() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert!((800..=950).contains(&records.len()));
        assert!(records.iter().all(|r| leaf(&r.codigo)));
        let seed: Vec<Nbs> = serde_json::from_slice(SEED)?;
        for codigo in ["1.0101.11.00", "1.0101.12.00", "1.1502.50.00"] {
            let found = records.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        Ok(())
    }

    #[test]
    fn prepara_planilha_oficial_e_propaga_404() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new([(URL.into(), Ok(FIXTURE.to_vec()))].into());
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].count, parse(FIXTURE)?.len() as u64);
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
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
