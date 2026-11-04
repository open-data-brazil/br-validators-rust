use super::{valid_checksum, CNPJ_LENGTH};
use crate::util::chars::{is_all_same_char, only_digits};

/// Verifica o CNPJ numérico após extrair os dígitos (BR-CNPJ-N-003).
pub fn is_valid_cnpj_numeric(input: &str) -> bool {
    let canonical = only_digits(input);
    canonical.len() == CNPJ_LENGTH && !is_all_same_char(&canonical) && valid_checksum(&canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cnpj::{CNPJ_GOLDEN_ALPHANUMERIC, CNPJ_GOLDEN_NUMERIC_MASKED};

    #[test]
    fn verifica_checksum_numerico() {
        assert!(is_valid_cnpj_numeric(CNPJ_GOLDEN_NUMERIC_MASKED));
        assert!(!is_valid_cnpj_numeric(CNPJ_GOLDEN_ALPHANUMERIC));
    }
}
