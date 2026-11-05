#![no_main]

use br_validators::boleto::{
    validate_arrecadacao, validate_codigo_barras, validate_linha_digitavel,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = br_validators::validate_boleto(s);
        let _ = validate_arrecadacao(s);
        let _ = validate_linha_digitavel(s);
        let _ = validate_codigo_barras(s);
    }
});
