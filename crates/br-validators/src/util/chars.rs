//! Operações de normalização de caracteres.

/// Mantém apenas os dígitos ASCII, na ordem original.
pub fn only_digits(input: &str) -> String {
    input.chars().filter(char::is_ascii_digit).collect()
}

/// Verifica se há pelo menos um caractere e todos são iguais.
pub fn is_all_same_char(input: &str) -> bool {
    let mut chars = input.chars();
    chars
        .next()
        .is_some_and(|first| chars.all(|current| current == first))
}

/// Remove apenas os caracteres indicados.
pub fn strip_chars(input: &str, removed: &[char]) -> String {
    input.chars().filter(|c| !removed.contains(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mantem_apenas_digitos_ascii() {
        assert_eq!(only_digits("a１２3.4"), "34");
    }

    #[test]
    fn detecta_caractere_repetido() {
        assert!(is_all_same_char("ççç"));
        assert!(!is_all_same_char("çc"));
        assert!(!is_all_same_char(""));
    }

    #[test]
    fn remove_apenas_caracteres_selecionados() {
        assert_eq!(strip_chars("a.b-ç", &['.', '-']), "abç");
    }
}
