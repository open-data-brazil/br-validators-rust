pub fn clean(text: &str) -> String {
    text.replace('\r', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn fold(text: &str) -> String {
    clean(text)
        .chars()
        .filter(|&ch| !matches!(ch, '\u{0300}'..='\u{036f}'))
        .flat_map(|ch| sem_acento_latino(ch).to_uppercase())
        .collect()
}

fn sem_acento_latino(ch: char) -> char {
    match ch {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => 'A',
        'ç' | 'Ç' => 'C',
        'è' | 'é' | 'ê' | 'ë' | 'È' | 'É' | 'Ê' | 'Ë' => 'E',
        'ì' | 'í' | 'î' | 'ï' | 'Ì' | 'Í' | 'Î' | 'Ï' => 'I',
        'ñ' | 'Ñ' => 'N',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' => 'O',
        'ù' | 'ú' | 'û' | 'ü' | 'Ù' | 'Ú' | 'Û' | 'Ü' => 'U',
        'ý' | 'ÿ' | 'Ý' | 'Ÿ' => 'Y',
        ch => ch,
    }
}

pub fn place_key(text: &str) -> String {
    let letters: String = fold(text)
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect();
    letters.split_whitespace().collect::<Vec<_>>().join(" ")
}
