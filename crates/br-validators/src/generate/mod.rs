//! Geração sintética para fixtures de teste (BR-GENERATE-001).

mod financial;
mod ie;
mod random;

pub use financial::{apply_arrecadacao_linha_mask, GeneratableCardBrand};

use std::{error::Error, fmt, str::FromStr};

use self::random::RandomSource;
use crate::{
    cnh::{cnh_check_digits, CNH_BASE_LENGTH},
    cnpj::{cnpj_check_digits, CNPJ_BASE_LENGTH},
    cpf::{cpf_check_digits, CPF_DV1_WEIGHTS},
    mask::mask,
    pis_pasep::PIS_PASEP_DV_WEIGHTS,
    platform::{DocumentType, PlatformOptions},
    renavam::{renavam_check_digit, RENAVAM_BASE_LENGTH},
    telefone::ANATEL_DDDS,
    titulo_eleitor::{
        titulo_check_digits, TITULO_ELEITOR_DV1_WEIGHTS_8, TITULO_ELEITOR_UF_BY_CODE,
    },
    util::{chars::is_all_same_char, modulo11::compute_check_digit},
    Uf,
};

const MAX_ATTEMPTS: usize = 50;

/// Mensagem usada enquanto a especificação de CPF alfanumérico estiver pendente.
pub const CPF_ALPHA_SPEC_PENDING: &str = "CPF_ALPHA_SPEC_PENDING: Alphanumeric CPF generation is pending official RFB specification — numeric CPF only until published";

/// Tipos para os quais há geração sintética no original.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratableDocumentType {
    /// CPF numérico.
    Cpf,
    /// CNPJ numérico ou alfanumérico.
    Cnpj,
    /// CEP.
    Cep,
    /// Placa de veículo.
    Placa,
    /// PIS ou PASEP.
    PisPasep,
    /// RENAVAM.
    Renavam,
    /// CNH.
    Cnh,
    /// Telefone.
    Telefone,
    /// Número de cartão.
    CartaoCredito,
    /// Inscrição estadual.
    InscricaoEstadual,
    /// Título de eleitor.
    TituloEleitor,
    /// Chave PIX EVP.
    Pix,
    /// Chave de acesso de NF-e.
    NfeChave,
    /// BR Code estático.
    Brcode,
    /// Boleto de cobrança.
    Boleto,
    /// Boleto de arrecadação.
    BoletoArrecadacao,
    /// Inscrição estadual de produtor rural paulista.
    InscricaoEstadualProdutorRural,
}

impl GeneratableDocumentType {
    /// Todos os tipos com geração disponível.
    pub const ALL: [Self; 17] = [
        Self::Cpf,
        Self::Cnpj,
        Self::Cep,
        Self::Placa,
        Self::PisPasep,
        Self::Renavam,
        Self::Cnh,
        Self::Telefone,
        Self::CartaoCredito,
        Self::InscricaoEstadual,
        Self::TituloEleitor,
        Self::Pix,
        Self::NfeChave,
        Self::Brcode,
        Self::Boleto,
        Self::BoletoArrecadacao,
        Self::InscricaoEstadualProdutorRural,
    ];

    /// Retorna o identificador original em kebab-case.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cpf => "cpf",
            Self::Cnpj => "cnpj",
            Self::Cep => "cep",
            Self::Placa => "placa",
            Self::PisPasep => "pis-pasep",
            Self::Renavam => "renavam",
            Self::Cnh => "cnh",
            Self::Telefone => "telefone",
            Self::CartaoCredito => "cartao-credito",
            Self::InscricaoEstadual => "inscricao-estadual",
            Self::TituloEleitor => "titulo-eleitor",
            Self::Pix => "pix",
            Self::NfeChave => "nfe-chave",
            Self::Brcode => "brcode",
            Self::Boleto => "boleto",
            Self::BoletoArrecadacao => "boleto-arrecadacao",
            Self::InscricaoEstadualProdutorRural => "inscricao-estadual-produtor-rural",
        }
    }
}

impl FromStr for GeneratableDocumentType {
    type Err = GenerateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|ty| ty.as_str() == input)
            .ok_or_else(|| {
                GenerateError::InvalidOption(format!("Unsupported generatable type: {input}"))
            })
    }
}

/// Formato selecionável do documento sintético.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateFormat {
    /// Documento numérico.
    Numeric,
    /// Documento alfanumérico.
    Alphanumeric,
    /// Placa legada.
    Legacy,
    /// Placa Mercosul.
    Mercosul,
    /// Telefone celular.
    Celular,
    /// Telefone fixo.
    Fixo,
}

impl FromStr for GenerateFormat {
    type Err = GenerateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "numeric" => Ok(Self::Numeric),
            "alphanumeric" => Ok(Self::Alphanumeric),
            "legacy" => Ok(Self::Legacy),
            "mercosul" => Ok(Self::Mercosul),
            "celular" => Ok(Self::Celular),
            "fixo" => Ok(Self::Fixo),
            _ => Err(GenerateError::InvalidOption(format!(
                "Unknown generation format: {input}"
            ))),
        }
    }
}

/// Opções para gerar um documento sintético.
#[derive(Debug, Default, Clone)]
pub struct GenerateOptions<'a> {
    /// Formato opcional.
    pub format: Option<GenerateFormat>,
    /// Aplica a máscara do tipo.
    pub masked: bool,
    /// Pede a forma canônica e tem precedência sobre `masked`.
    pub stripped: bool,
    /// Semente determinística. O algoritmo usa os 32 bits inferiores.
    pub seed: Option<u64>,
    /// UF exigida para inscrição estadual e título eleitoral.
    pub uf: Option<Uf>,
    /// Bandeira opcional para cartão de crédito.
    pub brand: Option<GeneratableCardBrand>,
    /// Chave PIX opcional do BR Code.
    pub pix_key: Option<&'a str>,
    /// Nome opcional do recebedor.
    pub merchant_name: Option<&'a str>,
    /// Cidade opcional do recebedor.
    pub merchant_city: Option<&'a str>,
    /// Valor decimal opcional do BR Code.
    pub amount: Option<&'a str>,
    /// Identificador opcional do BR Code.
    pub txid: Option<&'a str>,
}

/// Erro de configuração de geração.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerateError {
    /// A geração alfanumérica de CPF aguarda especificação.
    CpfAlphanumericPending,
    /// O tipo exige uma UF.
    MissingUf(GeneratableDocumentType),
    /// O valor de uma opção textual não é reconhecido.
    InvalidOption(String),
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CpfAlphanumericPending => f.write_str(CPF_ALPHA_SPEC_PENDING),
            Self::MissingUf(ty) => write!(f, "UF is required for {} generation", ty.as_str()),
            Self::InvalidOption(message) => f.write_str(message),
        }
    }
}

impl Error for GenerateError {}

fn random_base(rng: &mut RandomSource, length: usize) -> String {
    for _ in 0..MAX_ATTEMPTS {
        let base = rng.digits(length);
        if !is_all_same_char(&base) {
            return base;
        }
    }
    "123456789"
        .chars()
        .chain(std::iter::repeat('1'))
        .take(length)
        .collect()
}

fn cnpj(rng: &mut RandomSource, alpha: bool) -> String {
    let base = if alpha {
        (0..MAX_ATTEMPTS)
            .find_map(|_| {
                let base: String = (0..CNPJ_BASE_LENGTH)
                    .map(|_| rng.choose_char(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
                    .collect();
                (!is_all_same_char(&base)).then_some(base)
            })
            .unwrap_or_else(|| "12ABC34501DE".to_owned())
    } else {
        random_base(rng, CNPJ_BASE_LENGTH)
    };
    format!("{base}{}", cnpj_check_digits(&base))
}

fn placa(rng: &mut RandomSource, legacy: bool) -> String {
    let letters: String = (0..3).map(|_| rng.letter()).collect();
    if legacy {
        format!("{letters}{}", rng.digits(4))
    } else {
        format!(
            "{letters}{}{}{}",
            rng.digit(),
            rng.choose_char(b"ABCDEFGHIJ0123456789"),
            rng.digits(2)
        )
    }
}

fn telefone(rng: &mut RandomSource, fixo: bool) -> String {
    let ddd = rng.pick(&ANATEL_DDDS).unwrap_or("11");
    if fixo {
        format!("{ddd}{}{}", rng.int(2, 5), rng.digits(7))
    } else {
        format!("{ddd}9{}", rng.digits(8))
    }
}

fn titulo(rng: &mut RandomSource, uf: Uf) -> String {
    let code = TITULO_ELEITOR_UF_BY_CODE
        .iter()
        .position(|&state| state == uf)
        .and_then(|index| u8::try_from(index + 1).ok())
        .unwrap_or(1);
    let complete = |base: &str| {
        format!(
            "{base}{code:02}{}",
            titulo_check_digits(base, &TITULO_ELEITOR_DV1_WEIGHTS_8, code)
        )
    };
    for _ in 0..MAX_ATTEMPTS {
        let base = rng.digits(8);
        if !is_all_same_char(&base) {
            return complete(&base);
        }
    }
    match uf {
        Uf::SP => "000000000116".to_owned(),
        Uf::SC => "004356870906".to_owned(),
        _ => complete("12345678"),
    }
}

fn generate_canonical(
    ty: GeneratableDocumentType,
    opts: &GenerateOptions<'_>,
    rng: &mut RandomSource,
) -> Result<String, GenerateError> {
    use GeneratableDocumentType as D;

    Ok(match ty {
        D::Cpf => {
            if opts.format == Some(GenerateFormat::Alphanumeric) {
                return Err(GenerateError::CpfAlphanumericPending);
            }
            let base = random_base(rng, CPF_DV1_WEIGHTS.len());
            format!("{base}{}", cpf_check_digits(&base))
        }
        D::Cnpj => cnpj(rng, opts.format == Some(GenerateFormat::Alphanumeric)),
        D::Cep => (0..MAX_ATTEMPTS)
            .map(|_| rng.digits(8))
            .find(|cep| cep != "00000000")
            .unwrap_or_else(|| "01310100".to_owned()),
        D::Placa => placa(rng, opts.format == Some(GenerateFormat::Legacy)),
        D::PisPasep => {
            let base = random_base(rng, PIS_PASEP_DV_WEIGHTS.len());
            let dv = compute_check_digit(&base, &PIS_PASEP_DV_WEIGHTS, |c| {
                c.to_digit(10).unwrap_or_default()
            });
            format!("{base}{dv}")
        }
        D::Renavam => {
            let base = random_base(rng, RENAVAM_BASE_LENGTH);
            format!("{base}{}", renavam_check_digit(&base))
        }
        D::Cnh => {
            let base = random_base(rng, CNH_BASE_LENGTH);
            format!("{base}{}", cnh_check_digits(&base))
        }
        D::Telefone => telefone(rng, opts.format == Some(GenerateFormat::Fixo)),
        D::CartaoCredito => financial::cartao(rng, opts.brand),
        D::InscricaoEstadual => ie::generate_ie(rng, opts.uf.ok_or(GenerateError::MissingUf(ty))?),
        D::TituloEleitor => titulo(rng, opts.uf.ok_or(GenerateError::MissingUf(ty))?),
        D::Pix => financial::pix(rng),
        D::NfeChave => financial::nfe(rng),
        D::Brcode => financial::brcode(rng, opts),
        D::Boleto => financial::boleto(rng),
        D::BoletoArrecadacao => financial::arrecadacao(rng),
        D::InscricaoEstadualProdutorRural => ie::generate_rural(rng),
    })
}

/// Gera um documento sintético para testes. A semente fixa é reproduzível.
/// Sem semente, o gerador usa o relógio e um endereço de pilha.
///
/// # Errors
///
/// Retorna erro se faltar UF obrigatória ou se for solicitado CPF alfanumérico.
pub fn generate(
    ty: GeneratableDocumentType,
    opts: &GenerateOptions<'_>,
) -> Result<String, GenerateError> {
    let mut rng = RandomSource::new(opts.seed);
    let value = generate_canonical(ty, opts, &mut rng)?;
    if !opts.masked || opts.stripped {
        return Ok(value);
    }
    if ty == GeneratableDocumentType::BoletoArrecadacao {
        return Ok(apply_arrecadacao_linha_mask(&value));
    }
    let masked = ty
        .as_str()
        .parse::<DocumentType>()
        .ok()
        .and_then(|doc| mask(&value, doc, &PlatformOptions { uf: opts.uf }).ok());
    Ok(masked.unwrap_or(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejeita_opcoes_invalidas_sem_panico() {
        let opts = GenerateOptions {
            format: Some(GenerateFormat::Alphanumeric),
            seed: Some(42),
            ..GenerateOptions::default()
        };
        assert_eq!(
            generate(GeneratableDocumentType::Cpf, &opts)
                .unwrap_err()
                .to_string(),
            CPF_ALPHA_SPEC_PENDING
        );
        assert_eq!(
            generate(GeneratableDocumentType::TituloEleitor, &opts)
                .unwrap_err()
                .to_string(),
            "UF is required for titulo-eleitor generation"
        );
        assert!("rg".parse::<GeneratableDocumentType>().is_err());
    }
}
