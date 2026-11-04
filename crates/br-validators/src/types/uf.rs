use std::{error::Error, fmt, str::FromStr};

/// Unidade federativa brasileira.
///
/// Fonte: <https://www.ibge.gov.br/explica/codigos-dos-municipios.php>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Uf {
    /// Rondônia.
    RO = 11,
    /// Acre.
    AC = 12,
    /// Amazonas.
    AM = 13,
    /// Roraima.
    RR = 14,
    /// Pará.
    PA = 15,
    /// Amapá.
    AP = 16,
    /// Tocantins.
    TO = 17,
    /// Maranhão.
    MA = 21,
    /// Piauí.
    PI = 22,
    /// Ceará.
    CE = 23,
    /// Rio Grande do Norte.
    RN = 24,
    /// Paraíba.
    PB = 25,
    /// Pernambuco.
    PE = 26,
    /// Alagoas.
    AL = 27,
    /// Sergipe.
    SE = 28,
    /// Bahia.
    BA = 29,
    /// Minas Gerais.
    MG = 31,
    /// Espírito Santo.
    ES = 32,
    /// Rio de Janeiro.
    RJ = 33,
    /// São Paulo.
    SP = 35,
    /// Paraná.
    PR = 41,
    /// Santa Catarina.
    SC = 42,
    /// Rio Grande do Sul.
    RS = 43,
    /// Mato Grosso do Sul.
    MS = 50,
    /// Mato Grosso.
    MT = 51,
    /// Goiás.
    GO = 52,
    /// Distrito Federal.
    DF = 53,
}

impl Uf {
    /// As 27 UFs em ordem de código IBGE.
    pub const ALL: [Self; 27] = [
        Self::RO,
        Self::AC,
        Self::AM,
        Self::RR,
        Self::PA,
        Self::AP,
        Self::TO,
        Self::MA,
        Self::PI,
        Self::CE,
        Self::RN,
        Self::PB,
        Self::PE,
        Self::AL,
        Self::SE,
        Self::BA,
        Self::MG,
        Self::ES,
        Self::RJ,
        Self::SP,
        Self::PR,
        Self::SC,
        Self::RS,
        Self::MS,
        Self::MT,
        Self::GO,
        Self::DF,
    ];

    /// Sigla oficial em maiúsculas.
    pub const fn as_str(self) -> &'static str {
        self.attributes().0
    }

    /// Código IBGE de dois dígitos.
    pub const fn codigo_ibge(self) -> u8 {
        self as u8
    }

    /// Nome oficial da unidade federativa.
    pub const fn nome(self) -> &'static str {
        self.attributes().1
    }

    /// Nome da grande região do IBGE.
    pub const fn regiao(self) -> &'static str {
        self.attributes().2
    }

    /// Converte o código IBGE em UF.
    pub fn from_codigo_ibge(code: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|uf| uf.codigo_ibge() == code)
    }

    const fn attributes(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::RO => ("RO", "Rondônia", "Norte"),
            Self::AC => ("AC", "Acre", "Norte"),
            Self::AM => ("AM", "Amazonas", "Norte"),
            Self::RR => ("RR", "Roraima", "Norte"),
            Self::PA => ("PA", "Pará", "Norte"),
            Self::AP => ("AP", "Amapá", "Norte"),
            Self::TO => ("TO", "Tocantins", "Norte"),
            Self::MA => ("MA", "Maranhão", "Nordeste"),
            Self::PI => ("PI", "Piauí", "Nordeste"),
            Self::CE => ("CE", "Ceará", "Nordeste"),
            Self::RN => ("RN", "Rio Grande do Norte", "Nordeste"),
            Self::PB => ("PB", "Paraíba", "Nordeste"),
            Self::PE => ("PE", "Pernambuco", "Nordeste"),
            Self::AL => ("AL", "Alagoas", "Nordeste"),
            Self::SE => ("SE", "Sergipe", "Nordeste"),
            Self::BA => ("BA", "Bahia", "Nordeste"),
            Self::MG => ("MG", "Minas Gerais", "Sudeste"),
            Self::ES => ("ES", "Espírito Santo", "Sudeste"),
            Self::RJ => ("RJ", "Rio de Janeiro", "Sudeste"),
            Self::SP => ("SP", "São Paulo", "Sudeste"),
            Self::PR => ("PR", "Paraná", "Sul"),
            Self::SC => ("SC", "Santa Catarina", "Sul"),
            Self::RS => ("RS", "Rio Grande do Sul", "Sul"),
            Self::MS => ("MS", "Mato Grosso do Sul", "Centro-Oeste"),
            Self::MT => ("MT", "Mato Grosso", "Centro-Oeste"),
            Self::GO => ("GO", "Goiás", "Centro-Oeste"),
            Self::DF => ("DF", "Distrito Federal", "Centro-Oeste"),
        }
    }
}

/// Falha ao converter uma sigla em UF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseUfError;

impl fmt::Display for ParseUfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unknown UF code")
    }
}

impl Error for ParseUfError {}

impl fmt::Display for Uf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Uf {
    type Err = ParseUfError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|uf| uf.as_str().eq_ignore_ascii_case(input))
            .ok_or(ParseUfError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converte_siglas_e_codigos_das_27_ufs() {
        for uf in Uf::ALL {
            assert_eq!(uf.as_str().parse::<Uf>().unwrap(), uf);
            assert_eq!(uf.as_str().to_ascii_lowercase().parse::<Uf>().unwrap(), uf);
            assert_eq!(Uf::from_codigo_ibge(uf.codigo_ibge()), Some(uf));
            assert_eq!(uf.to_string(), uf.as_str());
            assert_ne!(uf.nome(), "");
            assert_ne!(uf.regiao(), "");
        }
        assert_eq!(Uf::SP.codigo_ibge(), 35);
        assert_eq!(Uf::SP.nome(), "São Paulo");
        assert_eq!(Uf::SP.regiao(), "Sudeste");
        assert_eq!(Uf::from_codigo_ibge(34), None);
        assert!("ZZ".parse::<Uf>().is_err());
    }
}
