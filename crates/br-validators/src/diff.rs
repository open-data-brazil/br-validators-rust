//! Comparação estrutural dos campos de documentos (BR-DIFF-001).

use crate::{
    compare::compare_with_options,
    platform::{DocumentType, PlatformOptions},
};

/// Campo com valor diferente após a normalização.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DiffField {
    /// Nome do campo no contrato original.
    pub field: &'static str,
    /// Valor na primeira entrada.
    #[cfg_attr(feature = "serde", serde(rename = "a"))]
    pub before: String,
    /// Valor na segunda entrada.
    #[cfg_attr(feature = "serde", serde(rename = "b"))]
    pub after: String,
}

/// Alterações observadas na comparação estrutural.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DiffResult {
    /// Indica se houve mudança em pelo menos um campo.
    pub changed: bool,
    /// Campos diferentes na ordem do documento.
    pub fields: Vec<DiffField>,
}

fn layout(ty: DocumentType) -> &'static [(&'static str, usize, usize)] {
    match ty {
        DocumentType::Cpf => &[("base", 0, 9), ("dv", 9, 11)],
        DocumentType::Cnpj => &[("base", 0, 12), ("dv", 12, 14)],
        DocumentType::Cep => &[("prefix", 0, 5), ("suffix", 5, 8)],
        DocumentType::Telefone => &[("ddd", 0, 2), ("subscriber", 2, usize::MAX)],
        DocumentType::PisPasep | DocumentType::Renavam => &[("base", 0, 10), ("dv", 10, 11)],
        DocumentType::Cnh => &[("base", 0, 9), ("dv1", 9, 10), ("dv2", 10, 11)],
        DocumentType::TituloEleitor => &[("sequential", 0, 8), ("uf", 8, 10), ("dv", 10, 12)],
        DocumentType::ProcessoJudicial => &[
            ("sequencial", 0, 7),
            ("checkDigits", 7, 9),
            ("ano", 9, 13),
            ("segmentoJustica", 13, 14),
            ("tribunal", 14, 16),
            ("origem", 16, 20),
        ],
        DocumentType::NfeChave => &[
            ("cUF", 0, 2),
            ("aamm", 2, 6),
            ("cnpj", 6, 20),
            ("mod", 20, 22),
            ("serie", 22, 25),
            ("nNF", 25, 34),
            ("tpEmis", 34, 35),
            ("cNF", 35, 43),
            ("cDV", 43, 44),
        ],
        _ => &[("value", 0, usize::MAX)],
    }
}

fn slice(value: &str, start: usize, end: usize) -> &str {
    value
        .get(start.min(value.len())..end.min(value.len()))
        .unwrap_or_default()
}

/// Identifica campos alterados com as opções padrão.
pub fn diff(a: &str, b: &str, ty: DocumentType) -> DiffResult {
    diff_with_options(a, b, ty, &PlatformOptions::default())
}

/// Identifica campos alterados com opções de normalização.
pub fn diff_with_options(a: &str, b: &str, ty: DocumentType, opts: &PlatformOptions) -> DiffResult {
    let compared = compare_with_options(a, b, ty, opts);
    if compared.equal {
        return DiffResult::default();
    }
    let fields: Vec<_> = layout(ty)
        .iter()
        .filter_map(|&(field, start, end)| {
            let before = slice(&compared.a_normalized, start, end);
            let after = slice(&compared.b_normalized, start, end);
            (before != after).then(|| DiffField {
                field,
                before: before.to_owned(),
                after: after.to_owned(),
            })
        })
        .collect();
    DiffResult {
        changed: !fields.is_empty(),
        fields,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Uf;

    #[test]
    fn relata_apenas_digito_verificador_alterado() {
        let result = diff("123.456.789-09", "12345678901", DocumentType::Cpf);
        assert_eq!(
            result.fields,
            [DiffField {
                field: "dv",
                before: "09".into(),
                after: "01".into()
            }]
        );
        assert!(result.changed);
        assert!(!diff("123.456.789-09", "12345678909", DocumentType::Cpf).changed);
    }

    #[test]
    fn compara_valores_incompletos_e_opacos() {
        let result = diff("12", "12345", DocumentType::Cep);
        assert_eq!(result.fields[0].field, "prefix");
        assert_eq!(result.fields[0].after, "12345");
        let result = diff("ABC1D23", "ABC1D24", DocumentType::Placa);
        assert_eq!(result.fields[0].field, "value");
        assert_eq!(result.fields[0].before, "ABC1D23");
    }

    #[test]
    fn usa_uf_para_normalizar_versoes_de_ie() {
        let result = diff_with_options(
            "00130000019",
            "130000019",
            DocumentType::InscricaoEstadual,
            &PlatformOptions { uf: Some(Uf::MT) },
        );
        assert_eq!(result, DiffResult::default());
    }
}
