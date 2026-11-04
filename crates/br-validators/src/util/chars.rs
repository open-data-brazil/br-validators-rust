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

// Conjunto usado por \s e trim() no TypeScript.
pub(crate) const fn is_ecmascript_whitespace(c: char) -> bool {
    matches!(c, '\u{9}'..='\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}'
        | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

pub(crate) fn format_groups(mut input: &str, groups: &[(usize, &str)]) -> Option<String> {
    let mut result = String::with_capacity(input.len());
    for &(width, separator) in groups {
        let (group, rest) = input.split_at_checked(width)?;
        result.push_str(group);
        result.push_str(separator);
        input = rest;
    }
    input.is_empty().then_some(result)
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
