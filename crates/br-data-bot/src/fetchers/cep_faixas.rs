use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Cursor},
    time::Duration,
};

use br_validators::schema::{Agendamento, CepFaixa, Municipio};
use zip::ZipArchive;

use crate::framework::{
    context::FetchContext,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::DatasetFetcher;

const BASE: &str = "https://ftp.ibge.gov.br/Cadastro_Nacional_de_Enderecos_para_Fins_Estatisticos/Censo_Demografico_2022/Arquivos_CNEFE/CSV/UF";
const ARCHIVES: &[&str] = &[
    "11_RO", "12_AC", "13_AM", "14_RR", "15_PA", "16_AP", "17_TO", "21_MA", "22_PI", "23_CE",
    "24_RN", "25_PB", "26_PE", "27_AL", "28_SE", "29_BA", "31_MG", "32_ES", "33_RJ", "35_SP",
    "41_PR", "42_SC", "43_RS", "50_MS", "51_MT", "52_GO", "53_DF",
];

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "cep-faixas",
    nome: "CEP prefix lookup (IBGE CNEFE 2022)",
    fonte: "IBGE CNEFE Censo 2022 microdata by UF",
    endpoints: &[BASE],
    agendamento: Agendamento::Manual,
    documentacao: "docs/OFFICIAL-SOURCES.md#cep-prefix-ranges",
    depends_on: &["ibge"],
    fetch,
};

// A ordem de inserção dos municípios decide empates, como no Map do TypeScript.
type Contagens = BTreeMap<u32, Vec<(u32, u64)>>;

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let municipios = ctx
        .read_embedded::<Municipio>("ibge", "municipios.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    if municipios.is_empty() {
        return Err(SourceError::Dependency(
            "Embedded IBGE municipios.json is required for CEP faixa enrichment".into(),
        ));
    }
    let opts = RequestOpts {
        timeout: Some(Duration::from_secs(600)),
        ..RequestOpts::default()
    };
    let mut contagens = Contagens::new();
    for arquivo in ARCHIVES {
        let url = format!("{BASE}/{arquivo}.zip");
        eprintln!("[cep-faixas] baixando {arquivo}.zip");
        let resposta = ctx.http.get(&url, &opts)?;
        eprintln!("[cep-faixas] agregando {arquivo}.csv");
        agrega_zip(&resposta.bytes, arquivo, &mut contagens)?;
    }
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "faixas.json",
        faixas(contagens, municipios),
        Bounds::new(20_000, 50_000),
    )?])
}

fn agrega_zip(bytes: &[u8], arquivo: &str, contagens: &mut Contagens) -> Result<(), SourceError> {
    let parse_error = |error| SourceError::Parse(format!("CNEFE {arquivo}.zip: {error}"));
    let mut zip = ZipArchive::new(Cursor::new(bytes)).map_err(parse_error)?;
    let csv = zip
        .by_name(&format!("{arquivo}.csv"))
        .map_err(parse_error)?;
    agrega_csv(BufReader::new(csv), contagens)
        .map_err(|error| SourceError::Parse(format!("CNEFE {arquivo}.csv: {error}")))
}

fn agrega_csv(mut csv: impl BufRead, contagens: &mut Contagens) -> std::io::Result<()> {
    let mut linha = Vec::new();
    csv.read_until(b'\n', &mut linha)?;
    loop {
        linha.clear();
        if csv.read_until(b'\n', &mut linha)? == 0 {
            break;
        }
        let mut colunas = linha.split(|&byte| byte == b';');
        let Some(codigo) = colunas.nth(2).and_then(codigo_municipio) else {
            continue;
        };
        let Some(cep) = colunas.nth(5) else { continue };
        let mut digitos = cep.iter().copied().filter(u8::is_ascii_digit);
        let Some(prefixo) = (0..5).try_fold(0, |prefixo, _| {
            digitos
                .next()
                .map(|digito| prefixo * 10 + u32::from(digito - b'0'))
        }) else {
            continue;
        };
        let municipios = contagens.entry(prefixo).or_default();
        match municipios
            .iter_mut()
            .find(|(municipio, _)| *municipio == codigo)
        {
            Some((_, total)) => *total += 1,
            None => municipios.push((codigo, 1)),
        }
    }
    Ok(())
}

fn codigo_municipio(raw: &[u8]) -> Option<u32> {
    let raw = std::str::from_utf8(raw).ok()?.trim_start();
    let raw = raw.strip_prefix('+').unwrap_or(raw);
    let len = raw.bytes().take_while(u8::is_ascii_digit).count();
    raw.get(..len)?.parse().ok()
}

fn faixas(contagens: Contagens, municipios: Vec<Municipio>) -> Vec<CepFaixa> {
    let municipios: BTreeMap<_, _> = municipios.into_iter().map(|m| (m.codigo, m)).collect();
    contagens
        .into_iter()
        .filter_map(|(prefixo, contagem)| {
            let (codigo, _) = contagem.into_iter().rev().max_by_key(|&(_, total)| total)?;
            let municipio = municipios.get(&codigo)?;
            Some(CepFaixa {
                prefixo: format!("{prefixo:05}"),
                uf: municipio.uf.clone(),
                codigo_ibge: codigo,
                cidade: municipio.nome.clone(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::{write::SimpleFileOptions, ZipWriter};

    #[test]
    fn agrega_csv_oficial_dentro_do_zip() -> anyhow::Result<()> {
        let csv = include_bytes!("../../tests/fixtures/cep-faixas/14_RR.csv");
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        zip.start_file("14_RR.csv", SimpleFileOptions::default())?;
        zip.write_all(csv)?;
        let mut contagens = Contagens::new();
        agrega_zip(&zip.finish()?.into_inner(), "14_RR", &mut contagens)?;
        assert_eq!(contagens[&69318], [(1400100, 2)]);
        let municipios = serde_json::from_slice(include_bytes!(
            "../../../br-validators/data/ibge/municipios.json"
        ))?;
        let seed: Vec<CepFaixa> = serde_json::from_slice(include_bytes!(
            "../../../br-validators/data/cep-faixas/faixas.json"
        ))?;
        let registros = faixas(contagens, municipios);
        assert_eq!(registros.len(), 25);
        for prefixo in ["69318", "69303"] {
            assert_eq!(
                registros.iter().find(|item| item.prefixo == prefixo),
                seed.iter().find(|item| item.prefixo == prefixo)
            );
            assert!(registros.iter().any(|item| item.prefixo == prefixo));
        }
        Ok(())
    }

    #[test]
    fn mantem_o_primeiro_municipio_no_empate() -> anyhow::Result<()> {
        let csv = b"header\na;b;3550308;d;e;f;g;h;01310-100\na;b;3304557;d;e;f;g;h;01310000\n";
        let mut contagens = Contagens::new();
        agrega_csv(csv.as_slice(), &mut contagens)?;
        let municipios = serde_json::from_slice(include_bytes!(
            "../../../br-validators/data/ibge/municipios.json"
        ))?;
        assert_eq!(faixas(contagens, municipios)[0].codigo_ibge, 3550308);
        Ok(())
    }
}
