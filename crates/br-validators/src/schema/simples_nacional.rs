/// Anexo do Simples Nacional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SimplesAnexo {
    /// Id.
    pub id: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
    /// Indica recolhimento da CPP fora do Simples.
    pub cpp_fora_simples: bool,
    /// Faixas.
    pub faixas: Vec<SimplesFaixa>,
}

impl super::Keyed for SimplesAnexo {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.id.clone()
    }
}

/// Faixa de receita bruta de um anexo do Simples Nacional.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SimplesFaixa {
    /// Faixa.
    pub faixa: u32,
    /// Receita bruta min.
    pub receita_bruta_min: f64,
    /// Receita bruta max.
    pub receita_bruta_max: f64,
    /// Aliquota nominal.
    pub aliquota_nominal: f64,
    /// Parcela deduzir.
    pub parcela_deduzir: f64,
}

impl super::Keyed for SimplesFaixa {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.faixa
    }
}
