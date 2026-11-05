//! Entrada posicional, arquivo ou stdin, na ordem da referência.

use std::{
    fs,
    io::{self, Read},
    path::Path,
};

/// Remove os espaços ECMAScript, incluindo o BOM.
pub fn trim_input(value: &str) -> &str {
    value.trim_matches(|c: char| c == '\u{feff}' || (c.is_whitespace() && c != '\u{85}'))
}
/// Lê os bytes como UTF-8 do Node, substituindo sequências inválidas.
pub fn read_file(path: &Path) -> Result<String, String> {
    let bytes = if path == Path::new("-") {
        let mut bytes = Vec::new();
        io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|_| "Cannot read stdin".to_owned())?;
        bytes
    } else {
        fs::read(path).map_err(|_| format!("Cannot read file: {}", path.display()))?
    };
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
/// Lê o arquivo antes de selecionar o argumento, como na referência.
pub fn resolve(value: Option<&str>, file: Option<&Path>) -> Result<Option<String>, String> {
    let content = file.map(read_file).transpose()?;
    let input = match value {
        Some("-") => content
            .filter(|_| file == Some(Path::new("-")))
            .map_or_else(|| read_file(Path::new("-")), Ok)?,
        Some(value) => return Ok((!value.is_empty()).then(|| value.to_owned())),
        None => match content {
            Some(content) => content,
            None => return Ok(None),
        },
    };
    let input = trim_input(&input);
    Ok((!input.is_empty()).then(|| input.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserva_espacos_posicionais_e_normaliza_arquivo() {
        assert_eq!(resolve(Some(" "), None).unwrap(), Some(" ".into()));
        assert_eq!(resolve(Some(""), None).unwrap(), None);
        assert_eq!(trim_input("\u{feff} value\r\n"), "value");
        assert_eq!(trim_input("\u{85}value\u{85}"), "\u{85}value\u{85}");
        assert!(resolve(
            Some("value"),
            Some(Path::new("/missing-br-validators-input"))
        )
        .is_err());
    }
}
