use br_validators::schema::{
    Agendamento, IrpfFaixaProgressiva, IrpfTabelaProgressiva, IrpfTabelas,
};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{static_source::probe, DatasetFetcher};

const URL: &str = "https://www.gov.br/receitafederal/pt-br/assuntos/meu-imposto-de-renda/tabelas";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "irpf",
    nome: "RFB IRPF — tabela progressiva mensal",
    fonte: "Receita Federal — Tabela progressiva mensal (static reference)",
    endpoints: &[
        URL,
        "https://www.gov.br/receitafederal/pt-br/assuntos/meu-imposto-de-renda/tabelas/tabela-progressiva-mensal",
        "https://www.planalto.gov.br/ccivil_03/leis/l7713.htm",
    ],
    agendamento: Agendamento::Estatico,
    documentacao: "docs/OFFICIAL-SOURCES.md#irpf",
    depends_on: &[],
    fetch,
};

fn tabela_2025() -> IrpfTabelaProgressiva {
    let faixas = [
        (1, 0.0, Some(2259.2), 0.0, 0.0, "Até 2.259,20"),
        (
            2,
            2259.21,
            Some(2826.65),
            0.075,
            169.44,
            "De 2.259,21 a 2.826,65",
        ),
        (
            3,
            2826.66,
            Some(3751.05),
            0.15,
            381.44,
            "De 2.826,66 a 3.751,05",
        ),
        (
            4,
            3751.06,
            Some(4664.68),
            0.225,
            662.77,
            "De 3.751,06 a 4.664,68",
        ),
        (5, 4664.69, None, 0.275, 896.0, "Acima de 4.664,68"),
    ]
    .into_iter()
    .map(
        |(faixa, base_calculo_min, base_calculo_max, aliquota, parcela_deduzir, descricao)| {
            IrpfFaixaProgressiva {
                faixa,
                base_calculo_min,
                base_calculo_max,
                aliquota,
                parcela_deduzir,
                descricao: descricao.into(),
            }
        },
    )
    .collect();
    IrpfTabelaProgressiva { ano: 2025, faixas }
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    probe(ctx, URL)?;
    let tabela = IrpfTabelas {
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
            "tabela-progressiva.json",
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

    const SEED: &[u8] = include_bytes!("../../../br-validators/data/irpf/tabela-progressiva.json");

    #[test]
    fn gera_tabela_da_referencia_e_retem_a_semente_se_probe_falha() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 26).expect("data da semente");
        let http = FakeHttp::new([(URL.into(), Ok(Vec::new()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].bytes, SEED);
        assert_eq!(files[0].contagens["faixas"], 5);
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
