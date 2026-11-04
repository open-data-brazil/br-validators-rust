/// Categoria de trabalhador da tabela 01 do eSocial.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EsocialCategoriaTrabalhador {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Grupo.
    pub grupo: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
    /// Início da vigência ou do período de referência.
    pub inicio: String,
    /// Fim da vigência, quando definido.
    pub termino: Option<String>,
}

impl super::Keyed for EsocialCategoriaTrabalhador {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}

/// Natureza de rubrica da tabela 03 do eSocial.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EsocialRubrica {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Natureza.
    pub natureza: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
    /// Início da vigência ou do período de referência.
    pub inicio: String,
    /// Fim da vigência, quando definido.
    pub termino: Option<String>,
    /// Código de incidência previdenciária da rubrica.
    #[serde(rename = "codIncCP")]
    pub cod_inc_cp: String,
}

impl super::Keyed for EsocialRubrica {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
