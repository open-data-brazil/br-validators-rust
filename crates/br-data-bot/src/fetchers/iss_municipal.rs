use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

use br_validators::schema::{Agendamento, IssMunicIbgeRow, IssMunicipalRow, Municipio};
use serde_json::Value;

use crate::framework::{
    context::FetchContext,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::fold,
    xlsx,
};

use super::{parse_json, DatasetFetcher};

mod seeds;

const PLANALTO: &str = "https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp116.htm";
const ART8: &str = "https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp116.htm#art8";
const SIDRA: &str = "https://apisidra.ibge.gov.br/values/t/5938/n6/all/v/37/p/2022";
const XLSX: &str =
    "https://ftp.ibge.gov.br/Pib_Municipios/2022_2023/xlsx/tabelas_completas_2022.xlsx";
const TOP: usize = 500;
const ISS_MIN: f64 = 2.0;
const ISS_MAX: f64 = 5.0;
const ISS_FILE: &str = "iss-municipal.json";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "iss-municipal",
    nome: "ISS municipal alíquotas — partial embed (capitals + top PIB)",
    fonte: "Municipal legislation + gov.br NFS-e registered alíquotas + LC 116/2003 Art. 8 band + IBGE SIDRA PIB municipal ranking",
    endpoints: &[
        PLANALTO,
        "https://www.ibge.gov.br/estatisticas/economicas/contas-nacionais/9088-produto-interno-bruto-dos-municipios.html",
        "https://www.ibge.gov.br/explica/codigos-dos-municipios.php",
        SIDRA,
        XLSX,
        "https://www.cnm.org.br/",
        "https://www.gov.br/nfse/pt-br",
        seeds::NFSE_URL,
        "https://ftp.ibge.gov.br/Perfil_Municipios/2024/Base_de_Dados/Base_MUNIC_2024_20251107.xlsx",
        "https://www.ibge.gov.br/estatisticas/economicas/contabilidade/9078-pesquisa-de-informacoes-municipais.html",
    ],
    agendamento: Agendamento::Manual,
    documentacao: "docs/OFFICIAL-SOURCES.md#iss-municipal",
    depends_on: &["ibge"],
    fetch,
};

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let municipios = ctx
        .read_embedded::<Municipio>("ibge", "municipios.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    if municipios.len() < 5500 {
        return Err(SourceError::Dependency(format!(
            "Expected at least 5500 IBGE municipios, got {}",
            municipios.len()
        )));
    }
    let opts = RequestOpts {
        timeout: Some(Duration::from_secs(120)),
        ..RequestOpts::default()
    };
    let ranking = parse_sidra(&ctx.http.get(SIDRA, &opts)?.bytes)?;
    if ranking.len() < 5000 {
        return Err(SourceError::Empty(format!(
            "Expected at least 5000 SIDRA municipal PIB rows, got {}",
            ranking.len()
        )));
    }
    let cross_check = ctx
        .http
        .get(XLSX, &RequestOpts::default())
        .and_then(|response| confere_planilha(&response.bytes, &municipios, &ranking));
    if let Err(error) = cross_check {
        // A conferência auxiliar não impede a publicação da resposta do SIDRA.
        ctx.retain_partial(SourceError::Probe(Box::new(error)));
    }
    let mut rows = seleciona(&municipios, &ranking, TOP, &ctx.today.to_string())?;
    if let Ok(anteriores) = ctx.read_embedded::<IssMunicipalRow>(FETCHER.id, ISS_FILE) {
        preserva_datas(&mut rows, anteriores);
    }
    let estimativas = rows.iter().filter(|row| row.estimativa).count();
    let munic = complemento_munic(&municipios, &rows);

    Ok(vec![
        ctx.prepare(FETCHER.id, ISS_FILE, rows, Bounds::new(TOP, TOP))?
            .with_count_key("municipios")
            .with_contagem("capitais", seeds::CAPITAIS.len())
            .with_contagem("estimativaRows", estimativas),
        ctx.prepare(
            FETCHER.id,
            "iss-munic-ibge.json",
            munic,
            Bounds::new(5000, 5600),
        )?
        .with_count_key("municIbgeMunicipios"),
    ])
}

fn parse_sidra(bytes: &[u8]) -> Result<Vec<(u32, f64)>, SourceError> {
    let payload: Vec<Value> = parse_json(bytes)?;
    let mut rows: Vec<_> = payload
        .iter()
        .filter_map(|row| {
            if row.get("NC")?.as_str()? == "Nível Territorial (Código)" {
                return None;
            }
            let codigo = row.get("D1C")?.as_str()?.trim().parse::<u32>().ok()?;
            let valor = row.get("V")?.as_str()?.replace('.', "");
            let valor = valor.trim();
            let pib = if valor.is_empty() {
                0.0
            } else {
                valor.parse::<f64>().ok()?
            };
            (codigo > 0 && pib.is_finite() && pib >= 0.0).then_some((codigo, pib))
        })
        .collect();
    // A ordenação estável mantém a ordem do SIDRA em caso de empate.
    rows.sort_by(|a, b| b.1.total_cmp(&a.1));
    Ok(rows)
}

fn confere_planilha(
    bytes: &[u8],
    municipios: &[Municipio],
    ranking: &[(u32, f64)],
) -> Result<(), SourceError> {
    let por_nome: BTreeMap<_, _> = municipios
        .iter()
        .map(|m| ((fold(&m.nome), m.uf.as_str()), m.codigo))
        .collect();
    let top: BTreeSet<_> = ranking.iter().take(20).map(|&(codigo, _)| codigo).collect();
    let planilha = xlsx::read_sheet(bytes, None)?;
    let amostra = planilha
        .iter()
        .filter_map(|row| {
            let rank = row.get(1)?.strip_suffix('º')?.trim_end();
            rank.parse::<u32>().ok()?;
            let (nome, uf) = row.first()?.strip_suffix(')')?.rsplit_once('(')?;
            let codigo = por_nome.get(&(fold(nome), uf))?;
            Some((nome, uf, codigo))
        })
        .take(10);
    for (nome, uf, codigo) in amostra {
        if !top.contains(codigo) {
            return Err(SourceError::Parse(format!(
                "XLSX cross-check failed: {nome}/{uf} not in SIDRA top 20"
            )));
        }
    }
    Ok(())
}

fn seleciona(
    municipios: &[Municipio],
    ranking: &[(u32, f64)],
    total: usize,
    data: &str,
) -> Result<Vec<IssMunicipalRow>, SourceError> {
    let por_codigo: BTreeMap<_, _> = municipios.iter().map(|m| (m.codigo, m)).collect();
    let fontes: BTreeMap<_, _> = seeds::NFSE
        .iter()
        .chain(seeds::CAPITAIS)
        .map(|&(codigo, min, max, url)| (codigo, (min, max, url)))
        .collect();
    let mut escolhidos = BTreeSet::new();
    let capitais = seeds::CAPITAIS
        .iter()
        .map(|&(codigo, _, _, _)| (codigo, None));
    let demais = ranking
        .iter()
        .enumerate()
        .map(|(rank, &(codigo, _))| (codigo, Some((rank + 1) as u32)));
    let mut rows = Vec::with_capacity(total);
    for (codigo, rank) in capitais.chain(demais) {
        if rows.len() >= total {
            break;
        }
        if !escolhidos.insert(codigo) {
            continue;
        }
        let m = por_codigo.get(&codigo).ok_or_else(|| {
            SourceError::Dependency(format!("IBGE code {codigo} missing from municipios index"))
        })?;
        let fonte = fontes.get(&codigo);
        let (aliquota_min, aliquota_max, url) = fonte.copied().unwrap_or((ISS_MIN, ISS_MAX, ART8));
        if !(ISS_MIN..=ISS_MAX).contains(&aliquota_min)
            || !(aliquota_min..=ISS_MAX).contains(&aliquota_max)
        {
            return Err(SourceError::Parse(format!(
                "Invalid ISS alíquota band for {}/{}: {aliquota_min}–{aliquota_max}",
                m.nome, m.uf
            )));
        }
        rows.push(IssMunicipalRow {
            codigo_ibge: codigo,
            nome: m.nome.clone(),
            uf: m.uf.clone(),
            aliquota_min,
            aliquota_max,
            lei_url: url.into(),
            capturado_em: data.into(),
            estimativa: fonte.is_none(),
            pib_rank: rank,
        });
    }
    if rows.len() != total {
        return Err(SourceError::Empty(format!(
            "Expected {total} ISS municipal rows, built {}",
            rows.len()
        )));
    }
    Ok(rows)
}

fn preserva_datas(rows: &mut [IssMunicipalRow], anteriores: Vec<IssMunicipalRow>) {
    let anteriores: BTreeMap<_, _> = anteriores
        .into_iter()
        .map(|row| (row.codigo_ibge, row))
        .collect();
    for row in rows {
        if let Some(anterior) = anteriores.get(&row.codigo_ibge) {
            let data_nova = std::mem::replace(&mut row.capturado_em, anterior.capturado_em.clone());
            if row != anterior {
                row.capturado_em = data_nova;
            }
        }
    }
}

fn complemento_munic(municipios: &[Municipio], rows: &[IssMunicipalRow]) -> Vec<IssMunicIbgeRow> {
    let escolhidos: BTreeSet<_> = rows.iter().map(|row| row.codigo_ibge).collect();
    // O MUNIC 2024 não publica alíquota por município. Aplica-se a faixa da LC 116.
    municipios
        .iter()
        .filter(|m| !escolhidos.contains(&m.codigo))
        .map(|m| IssMunicIbgeRow {
            codigo_ibge: m.codigo,
            nome: m.nome.clone(),
            uf: m.uf.clone(),
            aliquota_min: ISS_MIN,
            aliquota_max: ISS_MAX,
            munic_ano_pesquisa: 2024,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MUNICIPIOS: &[u8] = include_bytes!("../../../br-validators/data/ibge/municipios.json");
    const EMBEDDED: &[u8] =
        include_bytes!("../../../br-validators/data/iss-municipal/iss-municipal.json");
    const SIDRA_SAMPLE: &[u8] =
        include_bytes!("../../tests/fixtures/iss-municipal/sidra-pib-2022.json");
    const XLSX_SAMPLE: &[u8] =
        include_bytes!("../../tests/fixtures/iss-municipal/tabelas_completas_2022.xlsx");

    #[test]
    fn ordena_o_pib_do_sidra_e_confere_com_a_planilha_oficial() -> anyhow::Result<()> {
        let ranking = parse_sidra(SIDRA_SAMPLE)?;
        assert_eq!(ranking.len(), 50);
        assert_eq!(ranking[0].0, 3550308);
        assert_eq!(ranking[1].0, 3304557);
        let municipios: Vec<Municipio> = parse_json(MUNICIPIOS)?;
        confere_planilha(XLSX_SAMPLE, &municipios, &ranking)?;
        assert!(confere_planilha(XLSX_SAMPLE, &municipios, &[]).is_err());
        Ok(())
    }

    #[test]
    fn preserva_capitais_leis_confirmadas_e_faixas_estimadas() -> anyhow::Result<()> {
        let municipios: Vec<Municipio> = parse_json(MUNICIPIOS)?;
        let seed: Vec<IssMunicipalRow> = parse_json(EMBEDDED)?;
        let ranking = parse_sidra(SIDRA_SAMPLE)?;
        let rows = seleciona(&municipios, &ranking, 40, &seed[0].capturado_em)?;
        let capitais = rows
            .iter()
            .filter(|row| row.pib_rank.is_none() && !row.estimativa)
            .count();
        assert_eq!(capitais, 27);
        for (codigo, estimativa) in [(3550308, false), (3509502, false), (3518800, true)] {
            let row = rows
                .iter()
                .find(|row| row.codigo_ibge == codigo)
                .expect("município de referência");
            let anterior = seed
                .iter()
                .find(|row| row.codigo_ibge == codigo)
                .expect("semente");
            assert_eq!(row.estimativa, estimativa);
            assert_eq!(row.lei_url, anterior.lei_url);
            assert_eq!(row.pib_rank, anterior.pib_rank);
        }
        let munic = complemento_munic(&municipios, &seed);
        assert_eq!(munic.len() + seed.len(), municipios.len());
        assert_eq!(
            crate::framework::json_writer::canonical_json(&munic)?,
            include_bytes!("../../../br-validators/data/iss-municipal/iss-munic-ibge.json")
        );
        Ok(())
    }
}
