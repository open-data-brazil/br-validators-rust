//! Checksum usado no BR Code.

/// Calcula o CRC16-CCITT-FALSE (BR-BRC-001).
///
/// Fonte: <https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf>
pub fn crc16_ccitt_false(bytes: &[u8]) -> u16 {
    const POLYNOMIAL: u16 = 0x1021;
    const INITIAL: u16 = 0xFFFF;

    let mut crc = INITIAL;
    for &byte in bytes {
        crc ^= u16::from(byte) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 == 0 {
                crc << 1
            } else {
                (crc << 1) ^ POLYNOMIAL
            };
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_crc_da_sequencia_de_referencia() {
        assert_eq!(crc16_ccitt_false(b"123456789"), 0x29B1);
        assert_eq!(crc16_ccitt_false(b""), 0xFFFF);
    }
}
