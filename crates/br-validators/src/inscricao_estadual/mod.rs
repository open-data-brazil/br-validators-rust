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
pub use mt::{strip_ie_mt, validate_ie_mt};
pub use pa::{strip_ie_pa, validate_ie_pa};
pub use pb::{strip_ie_pb, validate_ie_pb};
pub use pe::{strip_ie_pe, validate_ie_pe};
pub use pi::{strip_ie_pi, validate_ie_pi};
pub use pr::{strip_ie_pr, validate_ie_pr};
pub use rj::{strip_ie_rj, validate_ie_rj};
pub use rn::{strip_ie_rn, validate_ie_rn};
pub use ro::{strip_ie_ro, validate_ie_ro};
pub use rr::{strip_ie_rr, validate_ie_rr};
pub use rs::{strip_ie_rs, validate_ie_rs};
pub use sc::{strip_ie_sc, validate_ie_sc};
pub use se::{strip_ie_se, validate_ie_se};
pub use sp::{strip_ie_sp, validate_ie_sp};
pub use to::{strip_ie_to, validate_ie_to};

use crate::types::{FormatResult, Uf};

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
        Uf::MT => validate_ie_mt(input),
        Uf::PA => validate_ie_pa(input),
        Uf::PB => validate_ie_pb(input),
        Uf::PE => validate_ie_pe(input),
        Uf::PI => validate_ie_pi(input),
        Uf::PR => validate_ie_pr(input),
        Uf::RJ => validate_ie_rj(input),
        Uf::RN => validate_ie_rn(input),
        Uf::RO => validate_ie_ro(input),
        Uf::RR => validate_ie_rr(input),
        Uf::RS => validate_ie_rs(input),
        Uf::SC => validate_ie_sc(input),
        Uf::SE => validate_ie_se(input),
        Uf::SP => validate_ie_sp(input),
        Uf::TO => validate_ie_to(input),
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
    match uf {
        Uf::DF => df::apply_mask(&valid.value),
        Uf::SP => sp::apply_mask(&valid.value),
        _ => Ok(valid.value.into_inner()),
    }
}
