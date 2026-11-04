//! Dígito verificador de Luhn para sequências de dígitos ASCII.

/// Verifica o dígito de Luhn (BR-LUHN-003).
///
/// Fonte: <https://www.iso.org/standard/70484.html>
pub fn luhn_valid(digits: &str) -> bool {
    !digits.is_empty() && luhn_remainder(digits, false) == Some(0)
}

/// Calcula o dígito que completa o payload. Retorna zero se houver outro caractere.
pub fn luhn_check_digit(payload: &str) -> u8 {
    luhn_remainder(payload, true).map_or(0, |remainder| (10 - remainder) % 10)
}

fn luhn_remainder(input: &str, double_first: bool) -> Option<u8> {
    input
        .bytes()
        .rev()
        .enumerate()
        .try_fold(0, |remainder, (position, byte)| {
            if !byte.is_ascii_digit() {
                return None;
            }

            let mut digit = byte - b'0';
            if (position % 2 == 0) == double_first {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            Some((remainder + digit) % 10)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifica_numero_com_digito_final() {
        assert!(luhn_valid("79927398713"));
        assert!(!luhn_valid("79927398714"));
        assert!(!luhn_valid("7992739871X"));
        assert!(!luhn_valid(""));
    }

    #[test]
    fn calcula_digito_para_completar_payload() {
        assert_eq!(luhn_check_digit("7992739871"), 3);
        assert_eq!(luhn_check_digit("18"), 2);
        assert_eq!(luhn_check_digit("X"), 0);
    }
}
