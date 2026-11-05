#![cfg(feature = "all-data")]

use std::{collections::BTreeSet, fs, path::Path};

use br_validators::pncp_reference::PncpReferenceTableId;
use br_validators::{
    data, get_dataset_metadata,
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

fn integridade_bytes(dir: &Path, arquivo: &str) -> Vec<u8> {
    let bytes = fs::read(dir.join(arquivo)).unwrap_or_else(|erro| panic!("{arquivo}: {erro}"));
    let texto = std::str::from_utf8(&bytes).expect("JSON precisa ser UTF-8");
    assert!(!texto.starts_with('\u{FEFF}'), "{arquivo}: BOM");
    assert!(!texto.contains('\r'), "{arquivo}: CR");
    assert!(texto.ends_with('\n'), "{arquivo}: newline final");
    assert!(!texto.contains('\u{FFFD}'), "{arquivo}: U+FFFD");
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

    for arquivo in &meta.arquivos {
        let bytes = integridade_bytes(&dir, &arquivo.path);
        assert_eq!(bytes.len() as u64, arquivo.bytes, "{}", arquivo.path);
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            arquivo.sha256,
            "{}: {}",
            meta.id,
            arquivo.path,
        );
    }
    let meta_bytes = integridade_bytes(&dir, "metadata.json");
    assert_eq!(
        serde_json::from_slice::<DatasetMetadata>(&meta_bytes).expect("JSON do metadata"),
        *meta,
    );
}

macro_rules! dataset {
    ($id:literal, $module:ident; $($arquivo:literal, $chave:literal, $dados:ident $(($($args:tt)*))?);+) => {
        #[test]
        fn $module() {
            use data::$module as dataset;
            let meta = get_dataset_metadata($id).expect("metadados do dataset");
            assert_eq!(meta.id, $id);
            arquivos(meta, &[$($arquivo),+]);
            $(
                let dados = &dataset::$dados$(($($args)*))?[..];
                contagem(meta, $chave, dados.len());
                ordenados(dados);
            )+
        }
    };
}

dataset!("aeroportos", aeroportos;
    "aeroportos.json", "aeroportos", get_all_aeroportos()
);

dataset!("anp-combustiveis", anp_combustiveis;
    "semanas.json", "semanas", get_all_anp_semanas_pesquisa();
    "precos-medios.json", "precosMedios", get_all_anp_precos_medios()
);

dataset!("bancos", bancos;
    "bancos.json", "bancos", get_all_bancos()
);

dataset!("cbo", cbo;
    "cbo.json", "cbo", get_all_cbo()
);

dataset!("cep-faixas", cep_faixas;
    "faixas.json", "faixas", FAIXAS
);

dataset!("cest", cest;
    "cest.json", "cest", get_all_cest()
);

dataset!("cfop", cfop;
    "cfop.json", "cfop", get_all_cfop()
);

dataset!("cnaes", cnaes;
    "cnaes.json", "cnaes", get_all_cnae()
);

dataset!("cnpj-motivos", cnpj_motivos;
    "motivos.json", "motivos", get_motivos_situacao_cadastral()
);

dataset!("csosn", csosn;
    "csosn.json", "csosn", get_all_csosn()
);

dataset!("cst", cst;
    "cst-icms.json", "icms", get_all_cst_icms();
    "cst-ipi.json", "ipi", get_all_cst_ipi();
    "cst-pis.json", "pis", get_all_cst_pis();
    "cst-cofins.json", "cofins", get_all_cst_cofins()
);

dataset!("esocial", esocial;
    "categorias.json", "categorias", get_all_esocial_categorias();
    "rubricas.json", "rubricas", get_all_esocial_rubricas()
);

dataset!("ibge", ibge;
    "estados.json", "estados", get_all_estados();
    "municipios.json", "municipios", get_all_municipios()
);

dataset!("ibpt", ibpt;
    "cargas.json", "cargas", get_all_ibpt_cargas()
);

dataset!("incoterms", incoterms;
    "incoterms.json", "incoterms", get_all_incoterms()
);

dataset!("iss-municipal", iss_municipal;
    "iss-municipal.json", "municipios", get_all_iss_municipal();
    "iss-munic-ibge.json", "municIbgeMunicipios", get_all_iss_munic_ibge()
);

dataset!("lc116", lc116;
    "lc116.json", "lc116", get_all_lc116()
);

dataset!("moedas", moedas;
    "moedas.json", "moedas", get_all_moedas()
);

dataset!("natureza-juridica", natureza_juridica;
    "naturezas.json", "naturezas", get_all_natureza_juridica()
);

dataset!("nbs", nbs;
    "nbs.json", "nbs", get_all_nbs()
);

dataset!("ncm", ncm;
    "ncm.json", "ncm", get_all_ncm()
);

dataset!("nfe-cuf", nfe_cuf;
    "cuf.json", "cuf", get_all_cuf()
);

dataset!("paises-bacen", paises_bacen;
    "paises.json", "paises", get_all_paises_bacen()
);

dataset!("pncp-reference", pncp_reference;
    "modalidades.json", "modalidades", get_all_pncp_reference(PncpReferenceTableId::Modalidades);
    "amparos-legais.json", "amparos-legais", get_all_pncp_reference(PncpReferenceTableId::AmparosLegais);
    "modos-disputa.json", "modos-disputa", get_all_pncp_reference(PncpReferenceTableId::ModosDisputa);
    "tipos-instrumentos-convocatorios.json", "tipos-instrumentos-convocatorios", get_all_pncp_reference(PncpReferenceTableId::TiposInstrumentosConvocatorios);
    "tipos-contrato.json", "tipos-contrato", get_all_pncp_reference(PncpReferenceTableId::TiposContrato);
    "criterios-julgamento.json", "criterios-julgamento", get_all_pncp_reference(PncpReferenceTableId::CriteriosJulgamento);
    "tipos-instrumentos-cobranca.json", "tipos-instrumentos-cobranca", get_all_pncp_reference(PncpReferenceTableId::TiposInstrumentosCobranca);
    "fontes-orcamentarias.json", "fontes-orcamentarias", get_all_pncp_reference(PncpReferenceTableId::FontesOrcamentarias)
);

dataset!("portos", portos;
    "portos.json", "portos", get_all_portos()
);

dataset!("ptax", ptax;
    "ptax.json", "cotacoes", get_ptax_list()
);

dataset!("selic", selic;
    "selic.json", "observacoes", get_selic_list()
);

dataset!("simples-nacional", simples_nacional;
    "anexos.json", "anexos", get_all_simples_anexos()
);

dataset!("telefone-ddd", telefone_ddd;
    "ddd-municipios.json", "ddds", get_ddd_municipios()
);

dataset!("tse-municipios", tse_municipios;
    "mapeamento.json", "municipios", get_mapeamento_tse_ibge()
);

#[test]
fn feriados() {
    use data::feriados as dataset;
    let meta = get_dataset_metadata("feriados").expect("metadados de feriados");
    assert_eq!(meta.id, "feriados");
    arquivos(meta, &["portaria-extras.json"]);
    contagem(
        meta,
        "portariaExtras",
        dataset::get_portaria_extras().values().map(Vec::len).sum(),
    );
    for (ano, dados) in dataset::get_portaria_extras() {
        ordenados(dados);
        let prefixo = format!("{ano:04}-");
        assert!(dados.iter().all(|dado| dado.data.starts_with(&prefixo)));
    }
}

#[test]
fn inss() {
    use data::inss as dataset;
    let meta = get_dataset_metadata("inss").expect("metadados de INSS");
    assert_eq!(meta.id, "inss");
    arquivos(meta, &["tabela-contribuicao.json"]);
    let tabelas = dataset::get_inss_tabelas();
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
    let meta = get_dataset_metadata("irpf").expect("metadados de IRPF");
    assert_eq!(meta.id, "irpf");
    arquivos(meta, &["tabela-progressiva.json"]);
    let tabelas = dataset::get_irpf_tabelas();
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
    let meta = get_dataset_metadata("transparencia-snapshots").expect("metadados da Transparência");
    assert_eq!(meta.id, "transparencia-snapshots");
    arquivos(meta, &["endpoints.json"]);
    let endpoints = dataset::get_transparencia_endpoints();
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
        get_dataset_metadata("aeroportos").expect("metadados de aeroportos"),
        "comIata",
        aeroportos::get_all_aeroportos()
            .iter()
            .filter(|a| a.iata.is_some())
            .count(),
    );
    let precos = anp_combustiveis::get_all_anp_precos_medios();
    contagem(
        get_dataset_metadata("anp-combustiveis").expect("metadados ANP"),
        "municipios",
        precos
            .iter()
            .map(|p| (&p.uf, &p.municipio_nome))
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(
        get_dataset_metadata("anp-combustiveis").expect("metadados ANP"),
        "produtos",
        precos
            .iter()
            .map(|p| &p.produto)
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(
        get_dataset_metadata("anp-combustiveis").expect("metadados ANP"),
        "ufs",
        precos.iter().map(|p| &p.uf).collect::<BTreeSet<_>>().len(),
    );
    contagem(
        get_dataset_metadata("iss-municipal").expect("metadados do ISS"),
        "estimativaRows",
        iss_municipal::get_all_iss_municipal()
            .iter()
            .filter(|m| m.estimativa)
            .count(),
    );
    contagem(
        get_dataset_metadata("ptax").expect("metadados de PTAX"),
        "diasUteis",
        ptax::get_ptax_list()
            .iter()
            .map(|c| &c.data)
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(
        get_dataset_metadata("ptax").expect("metadados de PTAX"),
        "moedas",
        ptax::get_ptax_list()
            .iter()
            .map(|c| &c.moeda)
            .collect::<BTreeSet<_>>()
            .len(),
    );
    contagem(
        get_dataset_metadata("selic").expect("metadados da Selic"),
        "dias",
        selic::get_selic_list().len(),
    );
    let anexos = simples_nacional::get_all_simples_anexos();
    contagem(
        get_dataset_metadata("simples-nacional").expect("metadados do Simples"),
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
