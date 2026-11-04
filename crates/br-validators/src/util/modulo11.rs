//! Cálculo do dígito verificador do CNPJ.

/// Mapeia o resto do módulo 11 conforme BR-CNPJ-A-004.
///
/// Fonte: <https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/perguntas-e-respostas/cnpj/cnpj-alfanumerico.pdf>
pub const fn modulo11_check_digit(sum: u32) -> u32 {
    let remainder = sum % 11;
    if remainder < 2 {
        0
    } else {
        11 - remainder
    }
}

/// Soma os produtos das posições com valor e peso, módulo 2³².
pub fn weighted_sum(values: &[u32], weights: &[u32]) -> u32 {
    values
        .iter()
        .zip(weights)
        .fold(0, |sum, (&value, &weight)| {
            sum.wrapping_add(value.wrapping_mul(weight))
        })
}

/// Converte os caracteres em valores e aplica os pesos e a regra do CNPJ.
pub fn compute_check_digit(chars: &str, weights: &[u32], value_fn: impl FnMut(char) -> u32) -> u32 {
    let values: Vec<_> = chars.chars().map(value_fn).collect();
    modulo11_check_digit(weighted_sum(&values, weights))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapeia_restos_do_cnpj() {
        assert_eq!(modulo11_check_digit(11), 0);
        assert_eq!(modulo11_check_digit(12), 0);
        assert_eq!(modulo11_check_digit(13), 9);
        assert_eq!(modulo11_check_digit(21), 1);
    }

    #[test]
    fn soma_valores_com_pesos() {
        assert_eq!(weighted_sum(&[1, 2, 3], &[4, 5, 6]), 32);
        assert_eq!(weighted_sum(&[1, 2], &[4]), 4);
        assert_eq!(weighted_sum(&[u32::MAX, 2], &[1, 1]), 1);
    }

    #[test]
    fn calcula_digito_com_conversao() {
        let weights = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        assert_eq!(
            compute_check_digit("112223330001", &weights, |c| c.to_digit(10).unwrap()),
            8
        );
    }
}
