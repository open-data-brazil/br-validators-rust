/// Região geográfica do IBGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IbgeRegiao {
    /// Id.
    pub id: u32,
    /// Nome publicado na fonte.
    pub nome: String,
}

impl super::Keyed for IbgeRegiao {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.id
    }
}

/// Unidade da Federação no cadastro do IBGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Estado {
    /// Código do registro na fonte.
    pub codigo: u32,
    /// Sigla.
    pub sigla: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Regiao.
    pub regiao: IbgeRegiao,
}

impl super::Keyed for Estado {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.codigo
    }
}

/// Município do cadastro de localidades do IBGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Municipio {
    /// Código do registro na fonte.
    pub codigo: u32,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
}

impl super::Keyed for Municipio {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.codigo
    }
}
