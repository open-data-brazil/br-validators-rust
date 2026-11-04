use std::collections::HashMap;

use br_validators::schema::{Aeroporto, Agendamento, Municipio};

use crate::framework::{
    context::FetchContext,
    csv,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::place_key,
};

use super::DatasetFetcher;

const URL: &str = "https://www.anac.gov.br/acesso-a-informacao/dados-abertos/areas-de-atuacao/aerodromos/lista-de-aerodromos-publicos/aerodromospublicosv1.csv/@@download/file/aerodromospublicosv1.csv";
const COLS: &[&str] = &["CÓDIGO OACI", "NOME", "MUNICÍPIO ATENDIDO", "UF"];

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "aeroportos",
    nome: "ANAC Public Aerodromos",
    fonte: "ANAC — Lista de aeródromos de uso público",
    endpoints: &[URL],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#aeroportos",
    depends_on: &["ibge"],
    fetch,
};

// Códigos complementares do mapa aeroportos-iata-oaci-map.ts da referência.
const IATA_OACI: &[(&str, &str)] = &[
    ("SBAA", "CDJ"),
    ("SBAC", "AQA"),
    ("SBAQ", "AQA"),
    ("SBAR", "AJU"),
    ("SBAT", "ATM"),
    ("SBBE", "BEL"),
    ("SBBH", "BHZ"),
    ("SBBR", "BSB"),
    ("SBBV", "BVB"),
    ("SBCA", "CAC"),
    ("SBCF", "CNF"),
    ("SBCG", "CGR"),
    ("SBCH", "XAP"),
    ("SBCJ", "CJZ"),
    ("SBCM", "CCM"),
    ("SBCN", "CAU"),
    ("SBCO", "OAL"),
    ("SBCP", "CFB"),
    ("SBCX", "CXJ"),
    ("SBCY", "CGB"),
    ("SBCZ", "CZS"),
    ("SBEG", "MAO"),
    ("SBFI", "IGU"),
    ("SBFL", "FLN"),
    ("SBFN", "FEN"),
    ("SBFZ", "FOR"),
    ("SBGL", "GIG"),
    ("SBGO", "GYN"),
    ("SBGR", "GRU"),
    ("SBGU", "GUZ"),
    ("SBHT", "ATM"),
    ("SBIH", "ITB"),
    ("SBJC", "JTC"),
    ("SBJD", "JDF"),
    ("SBJF", "JDF"),
    ("SBJI", "JPR"),
    ("SBJP", "JPA"),
    ("SBJR", "RIO"),
    ("SBJU", "JDO"),
    ("SBJV", "JOI"),
    ("SBKP", "VCP"),
    ("SBLJ", "LAJ"),
    ("SBLO", "LDB"),
    ("SBMA", "IMP"),
    ("SBME", "MEA"),
    ("SBMG", "MGF"),
    ("SBMK", "MOC"),
    ("SBML", "MII"),
    ("SBMN", "MAO"),
    ("SBMO", "MCZ"),
    ("SBMQ", "MCP"),
    ("SBMT", "SPX"),
    ("SBNF", "NVT"),
    ("SBNT", "NAT"),
    ("SBOI", "OUS"),
    ("SBPA", "POA"),
    ("SBPB", "JTC"),
    ("SBPC", "PCS"),
    ("SBPG", "PGZ"),
    ("SBPJ", "PMW"),
    ("SBPL", "PNZ"),
    ("SBPN", "PMG"),
    ("SBPP", "PPB"),
    ("SBPS", "BPS"),
    ("SBPV", "PVH"),
    ("SBRB", "RBR"),
    ("SBRF", "REC"),
    ("SBRJ", "SDU"),
    ("SBRP", "RAO"),
    ("SBSL", "SLZ"),
    ("SBSM", "RIA"),
    ("SBSN", "STM"),
    ("SBSO", "SOD"),
    ("SBSP", "CGH"),
    ("SBSR", "SJP"),
    ("SBST", "SSZ"),
    ("SBSV", "SSA"),
    ("SBTE", "THE"),
    ("SBTF", "TFF"),
    ("SBTL", "TUR"),
    ("SBUG", "URG"),
    ("SBUL", "UBA"),
    ("SBUR", "UDI"),
    ("SBVH", "BVH"),
    ("SBVT", "VIX"),
    ("SBYS", "QHP"),
    ("SWBC", "BAZ"),
    ("SWCA", "CAF"),
    ("SWCQ", "CCX"),
    ("SWGN", "GNM"),
    ("SWLC", "LEC"),
    ("SWNI", "NPR"),
    ("SWPM", "PIN"),
    ("SWRA", "RDC"),
    ("SWRP", "ROO"),
    ("SWST", "STZ"),
    ("SWTP", "TMT"),
    ("SWUA", "AUX"),
    ("SWVC", "VDC"),
    ("SWXM", "XAP"),
];

fn parse(bytes: &[u8], municipios: &[Municipio]) -> Result<Vec<Aeroporto>, SourceError> {
    let text = decode(bytes, Hint::Auto);
    let rows = csv::read(&text, b';', COLS)?;
    let index: HashMap<_, _> = municipios
        .iter()
        .map(|m| ((m.uf.as_str(), place_key(&m.nome)), m))
        .collect();
    let mut records = Vec::new();
    for mut row in rows {
        let icao = row.remove("CÓDIGO OACI").unwrap_or_default().to_uppercase();
        let nome = row.remove("NOME").unwrap_or_default();
        let uf = row.remove("UF").unwrap_or_default().to_uppercase();
        if icao.len() != 4
            || !icao.bytes().all(|b| b.is_ascii_uppercase())
            || nome.is_empty()
            || uf.len() != 2
        {
            continue;
        }
        let municipio_raw = row.remove("MUNICÍPIO ATENDIDO").unwrap_or_default();
        let municipio = index.get(&(uf.as_str(), place_key(&municipio_raw)));
        let municipio_nome = municipio.map_or(municipio_raw, |m| m.nome.clone());
        let municipio_ibge = municipio.map(|m| m.codigo);
        if municipio_nome.is_empty() {
            return Err(SourceError::Parse(format!(
                "ANAC: município ausente para {icao}"
            )));
        }
        let iata = IATA_OACI
            .binary_search_by_key(&icao.as_str(), |&(code, _)| code)
            .ok()
            .and_then(|index| IATA_OACI.get(index))
            .map(|&(_, iata)| iata.to_owned());
        records.push(Aeroporto {
            iata,
            icao,
            nome,
            uf,
            municipio_ibge,
            municipio_nome,
        });
    }
    Ok(records)
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let municipios: Vec<Municipio> = ctx
        .read_embedded("ibge", "municipios.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    let response = ctx.http.get(URL, &RequestOpts::default())?;
    let records = parse(&response.bytes, &municipios)?;
    let com_iata = records.iter().filter(|r| r.iata.is_some()).count();
    if com_iata < 50 {
        return Err(SourceError::Empty(format!(
            "ANAC: apenas {com_iata} aeródromos com IATA"
        )));
    }
    Ok(vec![ctx
        .prepare(
            FETCHER.id,
            "aeroportos.json",
            records,
            Bounds::new(400, 600),
        )?
        .with_contagem("comIata", com_iata)])
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/aeroportos/aerodromos.csv");
    const IBGE: &[u8] = include_bytes!("../../../br-validators/data/ibge/municipios.json");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/aeroportos/aeroportos.json");

    #[test]
    fn cruza_anac_com_o_mapa_iata_e_municipios_ibge() -> anyhow::Result<()> {
        let municipios: Vec<Municipio> = serde_json::from_slice(IBGE)?;
        let records = parse(FIXTURE, &municipios)?;
        let seed: Vec<Aeroporto> = serde_json::from_slice(SEED)?;
        for icao in ["SBGR", "SBGL", "SBBR"] {
            let found = records.iter().find(|r| r.icao == icao);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.icao == icao));
        }
        Ok(())
    }

    #[test]
    fn prepara_o_csv_com_dependencia_local_e_propaga_404() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let ibge = repo.path().join("crates/br-validators/data/ibge");
        fs::create_dir_all(&ibge)?;
        fs::write(ibge.join("municipios.json"), IBGE)?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new([(URL.into(), Ok(FIXTURE.to_vec()))].into());
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert!((400..=600).contains(&fetch(&ctx)?[0].count));
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
