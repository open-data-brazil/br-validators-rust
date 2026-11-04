use br_validators::schema::{Agendamento, SimplesAnexo};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{static_source::probe, DatasetFetcher};

const PLANALTO: &str = "https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp123.htm";
const ANEXO_I: &str =
    "http://normas.receita.fazenda.gov.br/sijut2consulta/anexoOutros.action?idArquivoBinario=48430";
const RESOLUCAO: &str =
    "http://normas.receita.fazenda.gov.br/sijut2consulta/link.action?visao=anotado&idAto=115262";
const FILE: &str = "anexos.json";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "simples-nacional",
    nome: "LC 123/2006 — Simples Nacional (anexos e faixas)",
    fonte: "Lei Complementar 123/2006 — Anexos I a V (redação LC 155/2016)",
    endpoints: &[PLANALTO, ANEXO_I, RESOLUCAO],
    agendamento: Agendamento::Manual,
    documentacao: "docs/OFFICIAL-SOURCES.md#simples-nacional-anexos",
    depends_on: &[],
    fetch,
};

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    probe(ctx, PLANALTO)?;
    // O script de referência valida a tabela mantida manualmente em anexos.json.
    let anexos = ctx
        .read_embedded::<SimplesAnexo>(FETCHER.id, FILE)
        .map_err(|error| SourceError::Parse(error.to_string()))?;
    for anexo in &anexos {
        if anexo.faixas.len() != 6 {
            return Err(SourceError::Parse(format!(
                "Anexo {}: expected 6 faixas, got {}",
                anexo.id,
                anexo.faixas.len()
            )));
        }
        if anexo.faixas.last().map(|faixa| faixa.receita_bruta_max) != Some(4_800_000.0) {
            return Err(SourceError::Parse(format!(
                "Anexo {}: last faixa must cap at R$ 4800000",
                anexo.id
            )));
        }
    }
    let faixas = anexos.iter().map(|anexo| anexo.faixas.len()).sum();
    Ok(vec![ctx
        .prepare(FETCHER.id, FILE, anexos, Bounds::new(5, 5))?
        .with_contagem("faixas", faixas)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;
    use std::fs;

    const SEED: &[u8] = include_bytes!("../../../br-validators/data/simples-nacional/anexos.json");

    #[test]
    fn valida_anexos_e_faixas_da_lei_complementar() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 25).expect("data da semente");
        let http = FakeHttp::new([(PLANALTO.into(), Ok(Vec::new()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let data_dir = ctx.data_dir(FETCHER.id);
        fs::create_dir_all(&data_dir)?;
        let path = data_dir.join(FILE);
        fs::write(&path, SEED)?;
        let files = fetch(&ctx)?;
        assert!(!files[0].changed);
        assert_eq!(files[0].contagens["faixas"], 30);

        let mut anexos: Vec<SimplesAnexo> = serde_json::from_slice(SEED)?;
        anexos[0].faixas.pop();
        fs::write(path, serde_json::to_vec(&anexos)?)?;
        assert!(matches!(fetch(&ctx), Err(SourceError::Parse(_))));
        Ok(())
    }
}
