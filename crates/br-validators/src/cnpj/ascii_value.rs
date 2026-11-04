// BR-CNPJ-A-002. A estrutura já assegura ASCII entre '0' e 'Z'.
pub(super) fn cnpj_char_value(character: char) -> u32 {
    u32::from(character) - u32::from('0')
}
