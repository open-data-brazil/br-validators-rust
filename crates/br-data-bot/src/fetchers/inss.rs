use br_validators::schema::{
    Agendamento, InssFaixaContribuicao, InssTabelaContribuicao, InssTabelas,
};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{static_source::probe, DatasetFetcher};

const URL: &str = "https://www.gov.br/inss/pt-br/direitos-e-deveres/inscricao-e-contribuicao/tabelas-de-contribuicao";
const TETO_2025: f64 = 8157.41;

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "inss",
    nome: "INSS — tabela progressiva de contribuição (empregado)",
    fonte: "Portaria Interministerial MPS/MF nº 6/2025 — Anexo II (static reference)",
    endpoints: &[
        "https://www.in.gov.br/web/dou/-/portaria-interministerial-mps/mf-n-6-de-10-de-janeiro-de-2025-606526848",
        "https://www.gov.br/previdencia/pt-br/assuntos/rpps/legislacao-dos-rpps/2025/PortariaInterministerialMPSMFn6de10jan2025.pdf",
        "https://www.gov.br/inss/pt-br/noticias/confira-como-ficaram-as-aliquotas-de-contribuicao-ao-inss",
        "https://www.planalto.gov.br/ccivil_03/_ato2004-2006/2004/lei/l10887.htm",
    ],
    agendamento: Agendamento::Estatico,
    documentacao: "docs/OFFICIAL-SOURCES.md#inss",
    depends_on: &[],
    fetch,
};

fn tabela_2025() -> InssTabelaContribuicao {
    let faixas = [
        (1, 0.0, 1518.0, 0.075, "Até 1.518,00"),
        (2, 1518.0, 2793.88, 0.09, "De 1.518,01 a 2.793,88"),
        (3, 2793.88, 4190.83, 0.12, "De 2.793,89 a 4.190,83"),
        (4, 4190.83, TETO_2025, 0.14, "De 4.190,84 a 8.157,41"),
    ]
    .into_iter()
    .map(
        |(faixa, salario_min, salario_max, aliquota, descricao)| InssFaixaContribuicao {
            faixa,
            salario_min,
            salario_max,
            aliquota,
            descricao: descricao.into(),
        },
    )
    .collect();
    InssTabelaContribuicao {
        ano: 2025,
        teto: TETO_2025,
        faixas,
    }
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    probe(ctx, URL)?;
    let tabela = InssTabelas {
        tabelas: vec![tabela_2025()],
    };
    let faixas = tabela
        .tabelas
        .iter()
        .map(|tabela| tabela.faixas.len())
        .sum();
    let file = ctx
        .prepare_document(
            FETCHER.id,
            "tabela-contribuicao.json",
            &tabela,
            |tabela| tabela.tabelas.iter().collect(),
            Bounds::new(1, usize::MAX),
        )?
        .with_count_key("anos")
        .with_contagem("faixas", faixas);
    Ok(vec![file])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const SEED: &[u8] = include_bytes!("../../../br-validators/data/inss/tabela-contribuicao.json");

    #[test]
    fn gera_as_faixas_e_o_teto_da_referencia_2025() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 26).expect("data da semente");
        let http = FakeHttp::new([(URL.into(), Ok(Vec::new()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].bytes, SEED);
        assert_eq!(files[0].contagens["faixas"], 4);
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
