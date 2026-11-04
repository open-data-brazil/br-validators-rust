//! Validação do RG conforme as regras da unidade federativa.

mod ac;
mod al;
mod am;
mod ap;
mod ba;
mod ce;
mod common;
mod df;
mod es;
mod go;
mod ma;
mod mg;
mod ms;
mod mt;
mod pa;
mod pb;
mod pe;
mod pi;
mod pr;
mod rj;
mod rn;
mod ro;
mod rr;
mod rs;
mod sc;
mod se;
mod sp;
mod to;

pub use ac::{strip_rg_ac, validate_rg_ac};
pub use al::{strip_rg_al, validate_rg_al};
pub use am::{strip_rg_am, validate_rg_am};
pub use ap::{strip_rg_ap, validate_rg_ap};
pub use ba::{strip_rg_ba, validate_rg_ba};
pub use ce::{strip_rg_ce, validate_rg_ce};
pub use common::{
    compute_rg_mod10_alternating_check_digit, RgDvAlgorithm, RgError, RgUfRules,
    RgValidationResult, ValidRg,
};
pub use df::{strip_rg_df, validate_rg_df};
pub use es::{strip_rg_es, validate_rg_es};
pub use go::{strip_rg_go, validate_rg_go};
pub use ma::{strip_rg_ma, validate_rg_ma};
pub use mg::{strip_rg_mg, validate_rg_mg};
pub use ms::{strip_rg_ms, validate_rg_ms};
pub use mt::{strip_rg_mt, validate_rg_mt};
pub use pa::{strip_rg_pa, validate_rg_pa};
pub use pb::{strip_rg_pb, validate_rg_pb};
pub use pe::{strip_rg_pe, validate_rg_pe};
pub use pi::{strip_rg_pi, validate_rg_pi};
pub use pr::{strip_rg_pr, validate_rg_pr};
pub use rj::{strip_rg_rj, validate_rg_rj};
pub use rn::{strip_rg_rn, validate_rg_rn};
pub use ro::{strip_rg_ro, validate_rg_ro};
pub use rr::{strip_rg_rr, validate_rg_rr};
pub use rs::{strip_rg_rs, validate_rg_rs};
pub use sc::{strip_rg_sc, validate_rg_sc};
pub use se::{strip_rg_se, validate_rg_se};
pub use sp::compute_rg_sp_check_digit;
pub use sp::{strip_rg_sp, validate_rg_sp};
pub use to::{strip_rg_to, validate_rg_to};

use crate::{
    types::{FormatResult, Uf, ValidationError, ValidationErrorCode as E},
    util::chars::{format_groups, only_digits},
};

/// UFs com validador de RG, na ordem do TypeScript.
pub const RG_SUPPORTED_UFS: [Uf; 27] = [
    Uf::SP,
    Uf::RJ,
    Uf::MG,
    Uf::PR,
    Uf::RS,
    Uf::SC,
    Uf::BA,
    Uf::AC,
    Uf::AL,
    Uf::AM,
    Uf::AP,
    Uf::DF,
    Uf::ES,
    Uf::GO,
    Uf::MA,
    Uf::MS,
    Uf::MT,
    Uf::PA,
    Uf::PB,
    Uf::CE,
    Uf::PE,
    Uf::PI,
    Uf::RN,
    Uf::RO,
    Uf::RR,
    Uf::SE,
    Uf::TO,
];
/// UFs pendentes no projeto de referência.
pub const RG_PENDING_UFS: [Uf; 0] = [];

/// Retorna as UFs com validador.
pub const fn get_rg_uf_support() -> &'static [Uf] {
    &RG_SUPPORTED_UFS
}

/// Retorna as UFs pendentes.
pub const fn get_rg_pending_ufs() -> &'static [Uf] {
    &RG_PENDING_UFS
}

/// Verifica se existe validador para a UF.
pub fn is_rg_uf_implemented(uf: Uf) -> bool {
    RG_SUPPORTED_UFS.contains(&uf)
}

/// Retorna as regras de validação da UF.
pub const fn get_rg_uf_rules(uf: Uf) -> &'static RgUfRules {
    match uf {
        Uf::AC => &ac::RULES,
        Uf::AL => &al::RULES,
        Uf::AM => &am::RULES,
        Uf::AP => &ap::RULES,
        Uf::BA => &ba::RULES,
        Uf::CE => &ce::RULES,
        Uf::DF => &df::RULES,
        Uf::ES => &es::RULES,
        Uf::GO => &go::RULES,
        Uf::MA => &ma::RULES,
        Uf::MG => &mg::RULES,
        Uf::MS => &ms::RULES,
        Uf::MT => &mt::RULES,
        Uf::PA => &pa::RULES,
        Uf::PB => &pb::RULES,
        Uf::PE => &pe::RULES,
        Uf::PI => &pi::RULES,
        Uf::PR => &pr::RULES,
        Uf::RJ => &rj::RULES,
        Uf::RN => &rn::RULES,
        Uf::RO => &ro::RULES,
        Uf::RR => &rr::RULES,
        Uf::RS => &rs::RULES,
        Uf::SC => &sc::RULES,
        Uf::SE => &se::RULES,
        Uf::SP => &sp::RULES,
        Uf::TO => &to::RULES,
    }
}

/// Retorna a fonte usada pelo validador estadual.
pub const fn get_rg_official_source_url(uf: Uf) -> &'static str {
    get_rg_uf_rules(uf).source_url
}

/// Remove a máscara conforme a regra estadual.
pub fn strip_rg(input: &str, uf: Uf) -> String {
    match uf {
        Uf::SP => strip_rg_sp(input),
        Uf::MG => strip_rg_mg(input),
        _ => only_digits(input),
    }
}

/// Valida o RG conforme a UF.
///
/// # Errors
///
/// Retorna erro se o documento violar uma regra estadual.
pub fn validate_rg(input: &str, uf: Uf) -> RgValidationResult {
    match uf {
        Uf::AC => validate_rg_ac(input),
        Uf::AL => validate_rg_al(input),
        Uf::AM => validate_rg_am(input),
        Uf::AP => validate_rg_ap(input),
        Uf::BA => validate_rg_ba(input),
        Uf::CE => validate_rg_ce(input),
        Uf::DF => validate_rg_df(input),
        Uf::ES => validate_rg_es(input),
        Uf::GO => validate_rg_go(input),
        Uf::MA => validate_rg_ma(input),
        Uf::MG => validate_rg_mg(input),
        Uf::MS => validate_rg_ms(input),
        Uf::MT => validate_rg_mt(input),
        Uf::PA => validate_rg_pa(input),
        Uf::PB => validate_rg_pb(input),
        Uf::PE => validate_rg_pe(input),
        Uf::PI => validate_rg_pi(input),
        Uf::PR => validate_rg_pr(input),
        Uf::RJ => validate_rg_rj(input),
        Uf::RN => validate_rg_rn(input),
        Uf::RO => validate_rg_ro(input),
        Uf::RR => validate_rg_rr(input),
        Uf::RS => validate_rg_rs(input),
        Uf::SC => validate_rg_sc(input),
        Uf::SE => validate_rg_se(input),
        Uf::SP => validate_rg_sp(input),
        Uf::TO => validate_rg_to(input),
    }
}

/// Verifica se o RG é válido na UF indicada.
pub fn is_valid_rg(input: &str, uf: Uf) -> bool {
    validate_rg(input, uf).is_ok()
}

/// Valida o RG e aplica a máscara da UF quando disponível.
///
/// # Errors
///
/// Retorna o erro de validação para um RG inválido.
pub fn format_rg(input: &str, uf: Uf) -> FormatResult {
    let canonical = validate_rg(input, uf)
        .map_err(|failure| failure.error)?
        .value;
    let (groups, message): (&[(usize, &str)], _) = match uf {
        Uf::SP => (
            &[(2, "."), (3, "."), (3, "-"), (1, "")],
            "RG must have 9 characters for SP-style mask",
        ),
        Uf::MG if canonical.starts_with('M') => (
            &[(2, "."), (3, "."), (3, "-"), (1, "")],
            "RG must have 8 digits for RJ-style mask",
        ),
        Uf::RJ | Uf::MG => (
            &[(1, "."), (3, "."), (3, "-"), (1, "")],
            "RG must have 8 digits for RJ-style mask",
        ),
        Uf::SC => (
            &[(3, "."), (3, "."), (3, "")],
            "RG must have 9 digits for SC mask",
        ),
        _ => return Ok(canonical.into_inner()),
    };
    format_groups(&canonical, groups).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: message.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distingue_mascara_e_nivel_de_validacao() {
        assert_eq!(format_rg("m2.799.811-1", Uf::MG).unwrap(), "M2.799.811-1");
        assert!(
            validate_rg("00.000.005-x", Uf::SP)
                .unwrap()
                .check_digit_validated
        );
        assert!(
            !validate_rg("1234/567890", Uf::RS)
                .unwrap()
                .check_digit_validated
        );
        assert!(validate_rg("000000", Uf::AC).is_ok());
        assert!(is_rg_uf_implemented(Uf::RR));
        assert_eq!(
            get_rg_official_source_url(Uf::RR),
            "https://www.policiacivil.rr.gov.br/"
        );
    }
}
