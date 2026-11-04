use encoding_rs::WINDOWS_1252;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hint {
    Auto,
    Utf8,
    Windows1252,
}

pub fn decode(bytes: &[u8], hint: Hint) -> String {
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
    match hint {
        Hint::Utf8 => String::from_utf8_lossy(bytes).into_owned(),
        Hint::Windows1252 => windows1252(bytes),
        Hint::Auto => match std::str::from_utf8(bytes) {
            Ok(text) => text.to_owned(),
            Err(_) => windows1252(bytes),
        },
    }
}

fn windows1252(bytes: &[u8]) -> String {
    WINDOWS_1252
        .decode_without_bom_handling(bytes)
        .0
        .into_owned()
}
