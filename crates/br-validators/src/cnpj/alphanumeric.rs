use super::{detect_canonical_format, valid_checksum};
use crate::util::chars::is_all_same_char;

/// Verifica o CNPJ sem máscara pelo algoritmo ASCII menos 48 (BR-CNPJ-A-002).
pub fn is_valid_cnpj_alphanumeric(input: &str) -> bool {
    let canonical = input.to_uppercase();
    detect_canonical_format(&canonical).is_some()
        && !is_all_same_char(&canonical)
        && valid_checksum(&canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cnpj::{
        CNPJ_GOLDEN_ALPHANUMERIC, CNPJ_GOLDEN_ALPHANUMERIC_MASKED, CNPJ_GOLDEN_NUMERIC,
    };

    #[test]
    fn verifica_checksum_de_base_alfanumerica() {
        assert!(is_valid_cnpj_alphanumeric(CNPJ_GOLDEN_ALPHANUMERIC));
        assert!(is_valid_cnpj_alphanumeric(CNPJ_GOLDEN_NUMERIC));
        assert!(!is_valid_cnpj_alphanumeric(CNPJ_GOLDEN_ALPHANUMERIC_MASKED));
    }
}
