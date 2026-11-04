use br_validators::schema::{Agendamento, Estado, NfeCuf};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{static_source::probe, DatasetFetcher};

const MANUAL: &str =
    "http://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=Il8k4BIjb48=";
const PORTAL: &str = "http://www.nfe.fazenda.gov.br/portal/principal.aspx";
const IBGE: &str = "https://www.ibge.gov.br/explica/codigos-dos-municipios.php";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "nfe-cuf",
    nome: "NF-e cUF — SEFAZ federative unit codes",
    fonte: "Manual NF-e — Tabela de UF (static reference)",
    endpoints: &[MANUAL, PORTAL, IBGE],
    agendamento: Agendamento::Estatico,
    documentacao: "docs/OFFICIAL-SOURCES.md#nfe-cuf",
    depends_on: &["ibge"],
    fetch,
};

// Fonte: Manual NF-e, Tabela de UF.
const UFS: &[(&str, &str, &str)] = &[
    ("11", "RO", "Rondônia"),
    ("12", "AC", "Acre"),
    ("13", "AM", "Amazonas"),
    ("14", "RR", "Roraima"),
    ("15", "PA", "Pará"),
    ("16", "AP", "Amapá"),
    ("17", "TO", "Tocantins"),
    ("21", "MA", "Maranhão"),
    ("22", "PI", "Piauí"),
    ("23", "CE", "Ceará"),
    ("24", "RN", "Rio Grande do Norte"),
    ("25", "PB", "Paraíba"),
    ("26", "PE", "Pernambuco"),
    ("27", "AL", "Alagoas"),
    ("28", "SE", "Sergipe"),
    ("29", "BA", "Bahia"),
    ("31", "MG", "Minas Gerais"),
    ("32", "ES", "Espírito Santo"),
    ("33", "RJ", "Rio de Janeiro"),
    ("35", "SP", "São Paulo"),
    ("41", "PR", "Paraná"),
    ("42", "SC", "Santa Catarina"),
    ("43", "RS", "Rio Grande do Sul"),
    ("50", "MS", "Mato Grosso do Sul"),
    ("51", "MT", "Mato Grosso"),
    ("52", "GO", "Goiás"),
    ("53", "DF", "Distrito Federal"),
];

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    probe(ctx, IBGE)?;
    let estados = ctx
        .read_embedded::<Estado>("ibge", "estados.json")
        .map_err(|error| SourceError::Parse(error.to_string()))?;
    let records = UFS
        .iter()
        .map(|&(codigo, uf, nome)| {
            let estado = estados
                .iter()
                .find(|estado| estado.sigla == uf)
                .ok_or_else(|| SourceError::Parse(format!("IBGE cross-ref missing UF {uf}")))?;
            if estado.codigo.to_string() != codigo || estado.nome != nome {
                return Err(SourceError::Parse(format!(
                    "NF-e cUF {codigo} does not match IBGE for {uf}"
                )));
            }
            Ok(NfeCuf {
                codigo: codigo.into(),
                uf: uf.into(),
                nome: nome.into(),
                codigo_ibge: codigo.into(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "cuf.json",
        records,
        Bounds::new(27, 27),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;
    use std::fs;

    #[test]
    fn cruza_a_tabela_do_manual_com_as_27_ufs_do_ibge() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 26).expect("data da semente");
        let http = FakeHttp::new([(IBGE.into(), Ok(Vec::new()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let ibge_dir = ctx.data_dir("ibge");
        fs::create_dir_all(&ibge_dir)?;
        let path = ibge_dir.join("estados.json");
        fs::write(
            &path,
            include_bytes!("../../../br-validators/data/ibge/estados.json"),
        )?;
        assert_eq!(
            fetch(&ctx)?[0].bytes,
            include_bytes!("../../../br-validators/data/nfe-cuf/cuf.json")
        );
        fs::write(path, b"[]")?;
        assert!(matches!(fetch(&ctx), Err(SourceError::Parse(_))));
        Ok(())
    }
}
