#![cfg(feature = "all-data")]

use std::{collections::BTreeSet, fs, path::Path};

use br_validators::{
    data,
    schema::{DatasetMetadata, Keyed},
};
use sha2::{Digest, Sha256};

fn ordenados<T: Keyed>(dados: &[T]) {
    for par in dados.windows(2) {
        if let [a, b] = par {
            let (a, b) = (a.key(), b.key());
            assert!(a < b, "chaves fora de ordem ou duplicadas: {a}, {b}");
        }
    }
}

fn contagem(meta: &DatasetMetadata, chave: &str, tamanho: usize) {
    assert_eq!(
        meta.contagens.get(chave).copied(),
        Some(tamanho as u64),
        "{}: contagem de {chave}",
        meta.id,
    );
}

fn integridade_bytes(dir: &Path, arquivo: &str, permite_substituicao: bool) -> Vec<u8> {
    let bytes = fs::read(dir.join(arquivo)).unwrap_or_else(|erro| panic!("{arquivo}: {erro}"));
    let texto = std::str::from_utf8(&bytes).expect("JSON precisa ser UTF-8");
    assert!(!texto.starts_with('\u{FEFF}'), "{arquivo}: BOM");
    assert!(!texto.contains('\r'), "{arquivo}: CR");
    assert!(texto.ends_with('\n'), "{arquivo}: newline final");
    if !permite_substituicao {
        assert!(!texto.contains('\u{FFFD}'), "{arquivo}: U+FFFD");
    }
    bytes
}

fn arquivos(meta: &DatasetMetadata, esperados: &[&str]) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(&meta.id);
    let listados = meta.arquivos.iter().map(|arquivo| arquivo.path.as_str());
    let mut previstos = esperados.to_vec();
    previstos.sort_unstable();
    assert!(listados.eq(previstos), "{}: lista de arquivos", meta.id);

    let nomes: BTreeSet<_> = fs::read_dir(&dir)
        .expect("diretório de dados")
        .map(|e| e.expect("entrada de diretório").file_name())
        .collect();
    let previstos = esperados
        .iter()
        .copied()
        .chain(["metadata.json"])
        .map(std::ffi::OsString::from)
        .collect();
    assert_eq!(nomes, previstos, "{}: arquivos em disco", meta.id);

    // TODO(FASE-08): remover após fetcher com Windows-1252.
    let permite_substituicao = ["cbo", "portos", "tse-municipios"].contains(&meta.id.as_str());
    for arquivo in &meta.arquivos {
        let bytes = integridade_bytes(&dir, &arquivo.path, permite_substituicao);
        assert_eq!(bytes.len() as u64, arquivo.bytes, "{}", arquivo.path);
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            arquivo.sha256,
            "{}: {}",
            meta.id,
            arquivo.path,
        );
    }
    let meta_bytes = integridade_bytes(&dir, "metadata.json", false);
    assert_eq!(
        serde_json::from_slice::<DatasetMetadata>(&meta_bytes).expect("JSON do metadata"),
        *meta,
    );
}

macro_rules! dataset {
    ($id:literal, $module:ident; $($arquivo:literal, $chave:literal, $dados:ident);+) => {
        #[test]
        fn $module() {
            use data::$module as dataset;
            let meta = &*dataset::METADATA;
            assert_eq!(meta.id, $id);
            arquivos(meta, &[$($arquivo),+]);
            $(
                contagem(meta, $chave, dataset::$dados.len());
                ordenados(&dataset::$dados);
            )+
        }
    };
}

dataset!("aeroportos", aeroportos;
    "aeroportos.json", "aeroportos", AEROPORTOS
);

dataset!("anp-combustiveis", anp_combustiveis;
    "semanas.json", "semanas", SEMANAS;
    "precos-medios.json", "precosMedios", PRECOS_MEDIOS
);

dataset!("bancos", bancos;
    "bancos.json", "bancos", BANCOS
);

dataset!("cbo", cbo;
    "cbo.json", "cbo", CBO
);

dataset!("cep-faixas", cep_faixas;
    "faixas.json", "faixas", FAIXAS
);

dataset!("cest", cest;
    "cest.json", "cest", CEST
);

dataset!("cfop", cfop;
    "cfop.json", "cfop", CFOP
);

dataset!("cnaes", cnaes;
    "cnaes.json", "cnaes", CNAES
);

dataset!("cnpj-motivos", cnpj_motivos;
    "motivos.json", "motivos", MOTIVOS
);

dataset!("csosn", csosn;
    "csosn.json", "csosn", CSOSN
);

dataset!("cst", cst;
    "cst-icms.json", "icms", CST_ICMS;
    "cst-ipi.json", "ipi", CST_IPI;
    "cst-pis.json", "pis", CST_PIS;
    "cst-cofins.json", "cofins", CST_COFINS
);

dataset!("esocial", esocial;
    "categorias.json", "categorias", CATEGORIAS;
    "rubricas.json", "rubricas", RUBRICAS
);

dataset!("ibge", ibge;
    "estados.json", "estados", ESTADOS;
    "municipios.json", "municipios", MUNICIPIOS
);

dataset!("ibpt", ibpt;
    "cargas.json", "cargas", CARGAS
);

dataset!("incoterms", incoterms;
    "incoterms.json", "incoterms", INCOTERMS
);

dataset!("iss-municipal", iss_municipal;
    "iss-municipal.json", "municipios", ISS_MUNICIPAL;
    "iss-munic-ibge.json", "municIbgeMunicipios", ISS_MUNIC_IBGE
);

dataset!("lc116", lc116;
    "lc116.json", "lc116", LC116
);

dataset!("moedas", moedas;
    "moedas.json", "moedas", MOEDAS
);

dataset!("natureza-juridica", natureza_juridica;
    "naturezas.json", "naturezas", NATUREZAS
);

dataset!("nbs", nbs;
    "nbs.json", "nbs", NBS
);

dataset!("ncm", ncm;
    "ncm.json", "ncm", NCM
);

dataset!("nfe-cuf", nfe_cuf;
    "cuf.json", "cuf", CUF
);

dataset!("paises-bacen", paises_bacen;
    "paises.json", "paises", PAISES
);

dataset!("pncp-reference", pncp_reference;
    "modalidades.json", "modalidades", MODALIDADES;
    "amparos-legais.json", "amparos-legais", AMPAROS_LEGAIS;
    "modos-disputa.json", "modos-disputa", MODOS_DISPUTA;
    "tipos-instrumentos-convocatorios.json", "tipos-instrumentos-convocatorios", TIPOS_INSTRUMENTOS_CONVOCATORIOS;
    "tipos-contrato.json", "tipos-contrato", TIPOS_CONTRATO;
    "criterios-julgamento.json", "criterios-julgamento", CRITERIOS_JULGAMENTO;
    "tipos-instrumentos-cobranca.json", "tipos-instrumentos-cobranca", TIPOS_INSTRUMENTOS_COBRANCA;
    "fontes-orcamentarias.json", "fontes-orcamentarias", FONTES_ORCAMENTARIAS
);

dataset!("portos", portos;
    "portos.json", "portos", PORTOS
);

dataset!("ptax", ptax;
    "ptax.json", "cotacoes", PTAX
);

dataset!("selic", selic;
    "selic.json", "observacoes", SELIC
);

dataset!("simples-nacional", simples_nacional;
    "anexos.json", "anexos", ANEXOS
);

dataset!("telefone-ddd", telefone_ddd;
    "ddd-municipios.json", "ddds", DDD_MUNICIPIOS
);

dataset!("tse-municipios", tse_municipios;
    "mapeamento.json", "municipios", MAPEAMENTO
);

#[test]
fn feriados() {
    use data::feriados as dataset;
    let meta = &*dataset::METADATA;
    assert_eq!(meta.id, "feriados");
    arquivos(meta, &["portaria-extras.json"]);
    contagem(
        meta,
        "portariaExtras",
        dataset::PORTARIA_EXTRAS.values().map(Vec::len).sum(),
    );
    for (ano, dados) in &*dataset::PORTARIA_EXTRAS {
        ordenados(dados);
        let prefixo = format!("{ano:04}-");
        assert!(dados.iter().all(|dado| dado.data.starts_with(&prefixo)));
    }
}

#[test]
fn inss() {
    use data::inss as dataset;
    let meta = &*dataset::METADATA;
    assert_eq!(meta.id, "inss");
    arquivos(meta, &["tabela-contribuicao.json"]);
    let tabelas = &dataset::TABELA_CONTRIBUICAO.tabelas;
    ordenados(tabelas);
    contagem(meta, "anos", tabelas.len());
    contagem(meta, "faixas", tabelas.iter().map(|t| t.faixas.len()).sum());
    for tabela in tabelas {
        ordenados(&tabela.faixas);
    }
}

#[test]
fn irpf() {
    use data::irpf as dataset;
    let meta = &*dataset::METADATA;
    assert_eq!(meta.id, "irpf");
    arquivos(meta, &["tabela-progressiva.json"]);
    let tabelas = &dataset::TABELA_PROGRESSIVA.tabelas;
    ordenados(tabelas);
    contagem(meta, "anos", tabelas.len());
    contagem(meta, "faixas", tabelas.iter().map(|t| t.faixas.len()).sum());
    for tabela in tabelas {
        ordenados(&tabela.faixas);
    }
}

#[test]
fn transparencia_snapshots() {
    use data::transparencia_snapshots as dataset;
    let meta = &*dataset::METADATA;
    assert_eq!(meta.id, "transparencia-snapshots");
    arquivos(meta, &["endpoints.json"]);
    let endpoints = &dataset::ENDPOINTS.endpoints;
    ordenados(endpoints);
    contagem(meta, "endpoints", endpoints.len());
    contagem(
        meta,
        "queryAdapter",
        endpoints
            .iter()
            .filter(|e| e.delivery == "query-adapter")
            .count(),
    );
}

#[test]
fn contagens_de_tabelas_e_subconjuntos() {
    use data::{aeroportos, anp_combustiveis, iss_municipal, ptax, selic, simples_nacional};

    contagem(
        &aeroportos::METADATA,
        "comIata",
        aeroportos::AEROPORTOS
            .iter()
            .filter(|a| a.iata.is_some())
            .count(),
    );
    let precos = &*anp_combustiveis::PRECOS_MEDIOS;
    contagem(
        &anp_combustiveis::METADATA,
        "municipios",
        precos
            .iter()
            .map(|p| (&p.uf, &p.municipio_nome))
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(
        &anp_combustiveis::METADATA,
        "produtos",
        precos
            .iter()
            .map(|p| &p.produto)
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(
        &anp_combustiveis::METADATA,
        "ufs",
        precos.iter().map(|p| &p.uf).collect::<BTreeSet<_>>().len(),
    );
    contagem(
        &iss_municipal::METADATA,
        "estimativaRows",
        iss_municipal::ISS_MUNICIPAL
            .iter()
            .filter(|m| m.estimativa)
            .count(),
    );
    contagem(
        &ptax::METADATA,
        "diasUteis",
        ptax::PTAX
            .iter()
            .map(|c| &c.data)
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(
        &ptax::METADATA,
        "moedas",
        ptax::PTAX
            .iter()
            .map(|c| &c.moeda)
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(&selic::METADATA, "dias", selic::SELIC.len());
    let anexos = &*simples_nacional::ANEXOS;
    contagem(
        &simples_nacional::METADATA,
        "faixas",
        anexos.iter().map(|a| a.faixas.len()).sum(),
    );
    for anexo in anexos {
        ordenados(&anexo.faixas);
    }
}

#[test]
fn catalogo_contem_os_34_datasets_em_ordem() {
    let catalogo = br_validators::get_data_catalog();
    assert_eq!(catalogo.len(), 34);
    assert!(catalogo.windows(2).all(|par| par[0].id < par[1].id));
}
