//! Inscrição estadual conforme o algoritmo de cada UF.

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
mod modulo_ie;
mod ms;

pub use ac::{strip_ie_ac, validate_ie_ac};
pub use al::{strip_ie_al, validate_ie_al};
pub use am::{strip_ie_am, validate_ie_am};
pub use ap::{strip_ie_ap, validate_ie_ap};
pub use ba::{strip_ie_ba, validate_ie_ba};
pub use ce::{strip_ie_ce, validate_ie_ce};
pub use common::{IeError, InscricaoEstadualValidationResult, ValidInscricaoEstadual};
pub use df::{strip_ie_df, validate_ie_df};
pub use es::{strip_ie_es, validate_ie_es};
pub use go::{strip_ie_go, validate_ie_go};
pub use ma::{strip_ie_ma, validate_ie_ma};
pub use mg::{strip_ie_mg, validate_ie_mg};
pub use ms::{strip_ie_ms, validate_ie_ms};

use crate::types::{FormatResult, Uf, ValidationErrorCode as E};

pub use crate::util::chars::only_digits as strip_inscricao_estadual;

/// Retorna a fonte oficial da regra estadual.
pub const fn get_ie_official_source_url(uf: Uf) -> &'static str {
    match uf {
        Uf::AC => "https://sefaz.ac.gov.br/",
        Uf::AL => "https://www.sefaz.al.gov.br/calculo",
        Uf::AM => "https://www.sefaz.am.gov.br/",
        Uf::AP => "https://www.sefaz.ap.gov.br/",
        Uf::BA => "https://www.sefaz.ba.gov.br/inspetoria-eletronica/icms/cadastro/calculo-dv/",
        Uf::CE => "https://www.sefaz.ce.gov.br/",
        Uf::DF => "https://www.receita.fazenda.df.gov.br/",
        Uf::ES => "https://sitenet.es.gov.br/sefaz/",
        Uf::GO => "http://www.sefaz.go.gov.br/ServicosAFA/ece.html",
        Uf::MA => "https://www.sefaz.ma.gov.br/",
        Uf::MG => "https://www.fazenda.mg.gov.br/empresas/Cadastro/cadastro/consultapublica.html",
        Uf::MS => "https://www.sefaz.ms.gov.br/",
        Uf::MT => "https://app1.sefaz.mt.gov.br/Sistema/legislacao/legislacaotribut.nsf/709f9c981a9d9f468425671300482be0/2217ddcf7a9b7cea03258c6c007324ba?OpenDocument=",
        Uf::PA => "https://www.sefa.pa.gov.br/",
        Uf::PB => "https://www.receita.pb.gov.br/",
        Uf::PE => "https://www.sefaz.pe.gov.br/",
        Uf::PI => "https://www.sefaz.pi.gov.br/",
        Uf::PR => "https://www.fazenda.pr.gov.br/Pagina/calculo-digito-verificador",
        Uf::RJ => "https://portal.fazenda.rj.gov.br/cadastro/",
        Uf::RN => "https://www.set.rn.gov.br/",
        Uf::RO => "https://www.sefin.ro.gov.br/",
        Uf::RR => "https://www.sefaz.rr.gov.br/",
        Uf::RS => "https://www.sefaz.rs.gov.br/",
        Uf::SC => "https://sat.sef.sc.gov.br/",
        Uf::SE => "https://www.sefaz.se.gov.br/",
        Uf::SP => "https://portal.fazenda.sp.gov.br/servicos/icms/Paginas/sintegra-rotina-consistencia.aspx",
        Uf::TO => "https://www.sefaz.to.gov.br/",
    }
}

/// Valida uma inscrição estadual na UF indicada.
///
/// # Errors
///
/// Retorna a primeira falha conforme as regras estaduais.
pub fn validate_inscricao_estadual(input: &str, uf: Uf) -> InscricaoEstadualValidationResult {
    match uf {
        Uf::AC => validate_ie_ac(input),
        Uf::AL => validate_ie_al(input),
        Uf::AM => validate_ie_am(input),
        Uf::AP => validate_ie_ap(input),
        Uf::BA => validate_ie_ba(input),
        Uf::CE => validate_ie_ce(input),
        Uf::DF => validate_ie_df(input),
        Uf::ES => validate_ie_es(input),
        Uf::GO => validate_ie_go(input),
        Uf::MA => validate_ie_ma(input),
        Uf::MG => validate_ie_mg(input),
        Uf::MS => validate_ie_ms(input),
        _ => Err(common::failure(
            None,
            E::UnsupportedFormat,
            format!("UF {uf} is not supported"),
        )),
    }
}

/// Verifica se a inscrição estadual é válida.
pub fn is_valid_inscricao_estadual(input: &str, uf: Uf) -> bool {
    validate_inscricao_estadual(input, uf).is_ok()
}

/// Valida e formata uma inscrição estadual.
///
/// # Errors
///
/// Retorna o erro de validação para uma inscrição inválida.
pub fn format_inscricao_estadual(input: &str, uf: Uf) -> FormatResult {
    let valid = validate_inscricao_estadual(input, uf).map_err(|failure| failure.error)?;
    if uf == Uf::DF {
        df::apply_mask(&valid.value)
    } else {
        Ok(valid.value.into_inner())
    }
}
