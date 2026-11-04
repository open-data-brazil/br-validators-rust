use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{ensure, Context, Result};
use br_validators::schema::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use br_data_bot::framework::json_writer::{write_dataset, write_json, WriteOutcome};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetadataOriginal {
    id: String,
    nome: String,
    fonte: String,
    endpoints: Vec<String>,
    capturado_em: String,
    contagens: BTreeMap<String, u64>,
    alteracoes: Alteracoes,
    documentacao: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TransparenciaOriginal {
    #[serde(rename = "capturadoEm")]
    _capturado_em: String,
    swagger_ok: bool,
    openapi_ok: bool,
    cadastro_url: String,
    adapter_package: String,
    endpoints: Vec<TransparenciaEndpoint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MunicOriginal {
    munic_base_url: String,
    munic_pesquisa_url: String,
    contagens: BTreeMap<String, u64>,
}

struct Importacao<'a> {
    origem: &'a Path,
    destino: &'a Path,
    metadata: DatasetMetadata,
    alterados: usize,
}

impl Importacao<'_> {
    fn ler<T: DeserializeOwned>(&self, arquivo: &str) -> Result<T> {
        ler_json(&self.origem.join(arquivo))
    }

    fn gravar<T: Serialize + ?Sized>(&mut self, arquivo: &str, value: &T) -> Result<()> {
        let meta = write_json(&self.destino.join(arquivo), value)?;
        self.registrar(meta);
        Ok(())
    }

    fn registrar(&mut self, outcome: WriteOutcome) {
        self.metadata.arquivos.push(outcome.arquivo);
        self.alterados += usize::from(outcome.changed);
    }

    fn contagem(&mut self, chave: &str, total: usize) -> Result<()> {
        let total = total as u64;
        if let Some(anterior) = self.metadata.contagens.insert(chave.into(), total) {
            ensure!(
                anterior == total,
                "{}: {chave} indica {anterior} registros, encontrados {total}",
                self.metadata.id
            );
        }
        Ok(())
    }

    fn lista<T: Keyed + DeserializeOwned + Serialize>(
        &mut self,
        arquivo: &str,
        contagem: &str,
    ) -> Result<()> {
        let mut dados: Vec<T> = self.ler(arquivo)?;
        ordenar(&mut dados)?;
        self.contagem(contagem, dados.len())?;
        let outcome = write_dataset(&self.destino.join(arquivo), &dados)?;
        self.registrar(outcome);
        Ok(())
    }
}

fn ler_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let bytes = fs::read(path).with_context(|| format!("ler {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("interpretar {}", path.display()))
}

fn ordenar<T: Keyed>(dados: &mut [T]) -> Result<()> {
    dados.sort_by_cached_key(Keyed::key);
    for par in dados.windows(2) {
        if let [a, b] = par {
            let chave = a.key();
            ensure!(chave != b.key(), "chave duplicada na semente: {chave}");
        }
    }
    Ok(())
}

fn importar(
    raiz: &Path,
    destino: &Path,
    id: &str,
    agenda: Agendamento,
    dados: impl FnOnce(&mut Importacao<'_>) -> Result<()>,
) -> Result<()> {
    let (dir, metadata) = match id {
        "cep-faixas" => ("core/cep/data", "faixa-metadata.json"),
        "telefone-ddd" => ("core/telefone/data", "ddd-metadata.json"),
        id => (id, "data/metadata.json"),
    };
    let origem = raiz.join(dir);
    let metadata_path = origem.join(metadata);
    let origem = metadata_path.parent().context("diretório de origem")?;
    let original: MetadataOriginal = ler_json(&metadata_path)?;
    ensure!(original.id == id, "metadata de {} em {id}", original.id);
    let destino = destino.join(id);
    fs::create_dir_all(&destino).with_context(|| format!("criar {}", destino.display()))?;
    let mut importacao = Importacao {
        origem,
        destino: &destino,
        metadata: DatasetMetadata {
            id: original.id,
            nome: original.nome,
            fonte: original.fonte,
            endpoints: original
                .endpoints
                .into_iter()
                .map(normalizar_endpoint)
                .collect(),
            capturado_em: original.capturado_em,
            contagens: original.contagens,
            alteracoes: original.alteracoes,
            agendamento: agenda,
            arquivos: Vec::new(),
            documentacao: original.documentacao,
        },
        alterados: 0,
    };
    dados(&mut importacao).with_context(|| format!("importar {id}"))?;
    importacao
        .metadata
        .arquivos
        .sort_by(|a, b| a.path.cmp(&b.path));
    write_json(&destino.join("metadata.json"), &importacao.metadata)?;
    println!(
        "{id}: {} arquivo(s), {} alterado(s)",
        importacao.metadata.arquivos.len(),
        importacao.alterados
    );
    Ok(())
}

fn normalizar_endpoint(endpoint: String) -> String {
    if let Some((_, espelho)) = endpoint.split_once("/data/source-mirrors/") {
        return format!("data/source-mirrors/{espelho}");
    }
    if endpoint == "packages/br-validators/src/ibge/data/municipios.json" {
        return "crates/br-validators/data/ibge/municipios.json".into();
    }
    endpoint
}

fn feriados(i: &mut Importacao<'_>) -> Result<()> {
    let mut dados: PortariaExtras = i.ler("portaria-extras.json")?;
    for ano in dados.values_mut() {
        ordenar(ano)?;
    }
    i.contagem("portariaExtras", dados.values().map(Vec::len).sum())?;
    i.gravar("portaria-extras.json", &dados)
}

fn inss(i: &mut Importacao<'_>) -> Result<()> {
    let mut dados: InssTabelas = i.ler("tabela-contribuicao.json")?;
    ordenar(&mut dados.tabelas)?;
    for tabela in &mut dados.tabelas {
        ordenar(&mut tabela.faixas)?;
    }
    i.contagem("anos", dados.tabelas.len())?;
    i.contagem("faixas", dados.tabelas.iter().map(|t| t.faixas.len()).sum())?;
    i.gravar("tabela-contribuicao.json", &dados)
}

fn irpf(i: &mut Importacao<'_>) -> Result<()> {
    let mut dados: IrpfTabelas = i.ler("tabela-progressiva.json")?;
    ordenar(&mut dados.tabelas)?;
    for tabela in &mut dados.tabelas {
        ordenar(&mut tabela.faixas)?;
    }
    i.contagem("anos", dados.tabelas.len())?;
    i.contagem("faixas", dados.tabelas.iter().map(|t| t.faixas.len()).sum())?;
    i.gravar("tabela-progressiva.json", &dados)
}

fn iss_municipal(i: &mut Importacao<'_>) -> Result<()> {
    let munic: MunicOriginal = i.ler("munic-ibge-metadata.json")?;
    i.metadata
        .endpoints
        .extend([munic.munic_base_url, munic.munic_pesquisa_url]);
    i.metadata.endpoints.dedup();
    let total = munic
        .contagens
        .get("municipios")
        .context("contagem MUNIC")?;
    i.metadata
        .contagens
        .insert("municIbgeMunicipios".into(), *total);
    i.lista::<IssMunicipalRow>("iss-municipal.json", "municipios")?;
    i.lista::<IssMunicIbgeRow>("iss-munic-ibge.json", "municIbgeMunicipios")
}

fn simples_nacional(i: &mut Importacao<'_>) -> Result<()> {
    let mut dados: Vec<SimplesAnexo> = i.ler("anexos.json")?;
    ordenar(&mut dados)?;
    for anexo in &mut dados {
        ordenar(&mut anexo.faixas)?;
    }
    i.contagem("anexos", dados.len())?;
    i.contagem("faixas", dados.iter().map(|a| a.faixas.len()).sum())?;
    i.gravar("anexos.json", &dados)
}

fn transparencia_snapshots(i: &mut Importacao<'_>) -> Result<()> {
    let mut original: TransparenciaOriginal = i.ler("endpoints.json")?;
    ordenar(&mut original.endpoints)?;
    i.contagem("endpoints", original.endpoints.len())?;
    i.gravar(
        "endpoints.json",
        &TransparenciaSnapshotsRegistry {
            swagger_ok: original.swagger_ok,
            openapi_ok: original.openapi_ok,
            cadastro_url: original.cadastro_url,
            adapter_package: original.adapter_package,
            endpoints: original.endpoints,
        },
    )
}

pub fn run(from: &Path) -> Result<()> {
    use Agendamento::{Derivado, Diario, Estatico, Manual, Semanal};

    let origem = from.join("packages/br-validators/src");
    let destino = Path::new("crates/br-validators/data");
    importar(&origem, destino, "aeroportos", Diario, |i| {
        i.lista::<Aeroporto>("aeroportos.json", "aeroportos")
    })?;
    importar(&origem, destino, "anp-combustiveis", Semanal, |i| {
        i.lista::<AnpSemanaPesquisa>("semanas.json", "semanas")?;
        i.lista::<AnpPrecoMedio>("precos-medios.json", "precosMedios")
    })?;
    importar(&origem, destino, "bancos", Diario, |i| {
        i.lista::<Banco>("bancos.json", "bancos")
    })?;
    importar(&origem, destino, "cbo", Diario, |i| {
        i.lista::<Cbo>("cbo.json", "cbo")
    })?;
    importar(&origem, destino, "cep-faixas", Manual, |i| {
        i.lista::<CepFaixa>("faixas.json", "faixas")
    })?;
    importar(&origem, destino, "cest", Diario, |i| {
        i.lista::<Cest>("cest.json", "cest")
    })?;
    importar(&origem, destino, "cfop", Diario, |i| {
        i.lista::<Cfop>("cfop.json", "cfop")
    })?;
    importar(&origem, destino, "cnaes", Diario, |i| {
        i.lista::<Cnae>("cnaes.json", "cnaes")
    })?;
    importar(&origem, destino, "cnpj-motivos", Diario, |i| {
        i.lista::<MotivoSituacaoCadastral>("motivos.json", "motivos")
    })?;
    importar(&origem, destino, "csosn", Estatico, |i| {
        i.lista::<Csosn>("csosn.json", "csosn")
    })?;
    importar(&origem, destino, "cst", Manual, |i| {
        i.lista::<Cst>("cst-icms.json", "icms")?;
        i.lista::<Cst>("cst-ipi.json", "ipi")?;
        i.lista::<Cst>("cst-pis.json", "pis")?;
        i.lista::<Cst>("cst-cofins.json", "cofins")
    })?;
    importar(&origem, destino, "esocial", Manual, |i| {
        i.lista::<EsocialCategoriaTrabalhador>("categorias.json", "categorias")?;
        i.lista::<EsocialRubrica>("rubricas.json", "rubricas")
    })?;
    importar(&origem, destino, "feriados", Derivado, feriados)?;
    importar(&origem, destino, "ibge", Diario, |i| {
        i.lista::<Estado>("estados.json", "estados")?;
        i.lista::<Municipio>("municipios.json", "municipios")
    })?;
    importar(&origem, destino, "ibpt", Manual, |i| {
        i.lista::<IbptCargaTributaria>("cargas.json", "cargas")
    })?;
    importar(&origem, destino, "incoterms", Estatico, |i| {
        i.lista::<Incoterm>("incoterms.json", "incoterms")
    })?;
    importar(&origem, destino, "inss", Estatico, inss)?;
    importar(&origem, destino, "irpf", Estatico, irpf)?;
    importar(&origem, destino, "iss-municipal", Manual, iss_municipal)?;
    importar(&origem, destino, "lc116", Manual, |i| {
        i.lista::<Lc116>("lc116.json", "lc116")
    })?;
    importar(&origem, destino, "moedas", Diario, |i| {
        i.lista::<Moeda>("moedas.json", "moedas")
    })?;
    importar(&origem, destino, "natureza-juridica", Diario, |i| {
        i.lista::<NaturezaJuridica>("naturezas.json", "naturezas")
    })?;
    importar(&origem, destino, "nbs", Diario, |i| {
        i.lista::<Nbs>("nbs.json", "nbs")
    })?;
    importar(&origem, destino, "ncm", Diario, |i| {
        i.lista::<Ncm>("ncm.json", "ncm")
    })?;
    importar(&origem, destino, "nfe-cuf", Estatico, |i| {
        i.lista::<NfeCuf>("cuf.json", "cuf")
    })?;
    importar(&origem, destino, "paises-bacen", Diario, |i| {
        i.lista::<PaisBacen>("paises.json", "paises")
    })?;
    importar(&origem, destino, "pncp-reference", Diario, |i| {
        i.lista::<PncpReferenceItem>("modalidades.json", "modalidades")?;
        i.lista::<PncpReferenceItem>("amparos-legais.json", "amparos-legais")?;
        i.lista::<PncpReferenceItem>("modos-disputa.json", "modos-disputa")?;
        i.lista::<PncpReferenceItem>(
            "tipos-instrumentos-convocatorios.json",
            "tipos-instrumentos-convocatorios",
        )?;
        i.lista::<PncpReferenceItem>("tipos-contrato.json", "tipos-contrato")?;
        i.lista::<PncpReferenceItem>("criterios-julgamento.json", "criterios-julgamento")?;
        i.lista::<PncpReferenceItem>(
            "tipos-instrumentos-cobranca.json",
            "tipos-instrumentos-cobranca",
        )?;
        i.lista::<PncpReferenceItem>("fontes-orcamentarias.json", "fontes-orcamentarias")
    })?;
    importar(&origem, destino, "portos", Diario, |i| {
        i.lista::<Porto>("portos.json", "portos")
    })?;
    importar(&origem, destino, "ptax", Diario, |i| {
        i.lista::<PtaxCotacao>("ptax.json", "cotacoes")
    })?;
    importar(&origem, destino, "selic", Diario, |i| {
        i.lista::<SelicMetaObservacao>("selic.json", "observacoes")
    })?;
    importar(
        &origem,
        destino,
        "simples-nacional",
        Manual,
        simples_nacional,
    )?;
    importar(&origem, destino, "telefone-ddd", Derivado, |i| {
        i.lista::<DddInfo>("ddd-municipios.json", "ddds")
    })?;
    importar(
        &origem,
        destino,
        "transparencia-snapshots",
        Diario,
        transparencia_snapshots,
    )?;
    importar(&origem, destino, "tse-municipios", Diario, |i| {
        i.lista::<TseMunicipioMapping>("mapeamento.json", "municipios")
    })?;
    Ok(())
}
