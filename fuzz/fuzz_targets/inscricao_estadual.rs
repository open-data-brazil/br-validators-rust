#![no_main]

use br_validators::Uf;
use libfuzzer_sys::fuzz_target;

const ALL_UFS: [Uf; 27] = [
    Uf::AC,
    Uf::AL,
    Uf::AP,
    Uf::AM,
    Uf::BA,
    Uf::CE,
    Uf::DF,
    Uf::ES,
    Uf::GO,
    Uf::MA,
    Uf::MT,
    Uf::MS,
    Uf::MG,
    Uf::PA,
    Uf::PB,
    Uf::PR,
    Uf::PE,
    Uf::PI,
    Uf::RJ,
    Uf::RN,
    Uf::RS,
    Uf::RO,
    Uf::RR,
    Uf::SC,
    Uf::SP,
    Uf::SE,
    Uf::TO,
];

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    let uf_idx = (data[0] as usize) % ALL_UFS.len();
    let uf = ALL_UFS[uf_idx];
    if let Ok(s) = std::str::from_utf8(&data[1..]) {
        let _ = br_validators::validate_inscricao_estadual(s, uf);
        if uf == Uf::SP {
            let _ = br_validators::validate_ie_produtor_rural(s, uf);
        }
    }
});
