use std::{fmt, ops::Deref};

macro_rules! branded {
    ($($(#[$meta:meta])* $name:ident $(=> $constructor:ident)?),* $(,)?) => {$(
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
        pub struct $name(String);

        impl $name {
            $(
                pub(crate) fn $constructor(value: String) -> Self {
                    Self(value)
                }
            )?

            /// Empresta a representação canônica.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consome o documento e retorna sua representação canônica.
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }
    )*};
}

branded!(
    /// CNPJ validado.
    Cnpj => new_unchecked,
    /// CPF validado.
    Cpf => new_unchecked,
    /// CNH validada.
    Cnh,
    /// RENAVAM validado.
    Renavam,
    /// Título de eleitor validado.
    TituloEleitor,
    /// Chave de acesso de NF-e ou NFC-e validada.
    NfeChave,
    /// Número de processo judicial validado.
    ProcessoJudicial,
    /// CEP validado.
    Cep,
    /// Placa de veículo validada.
    Placa,
    /// PIS ou PASEP validado.
    PisPasep,
    /// Número de Identificação do Trabalhador validado.
    Nit,
    /// Chave PIX validada.
    PixKey,
    /// Linha digitável de boleto de cobrança validada.
    LinhaDigitavel,
    /// Código de barras de boleto de cobrança validado.
    CodigoBarras,
    /// Boleto de arrecadação validado.
    Arrecadacao,
    /// Número de cartão validado.
    CartaoCredito,
    /// Código EAN validado.
    Ean,
    /// Inscrição estadual validada.
    InscricaoEstadual,
    /// Inscrição estadual de produtor rural validada.
    InscricaoEstadualProdutorRural,
    /// Registro geral validado.
    Rg,
    /// Número de telefone validado.
    Telefone,
    /// Payload BR Code validado.
    BrCode,
    /// Identificador CNIS validado.
    Cnis,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oferece_representacao_canonica() {
        let cpf = Cpf(String::from("52998224725"));
        assert_eq!(cpf.as_str(), "52998224725");
        assert_eq!(cpf.as_ref(), "52998224725");
        assert_eq!(cpf, "52998224725");
        assert_eq!(cpf.to_string(), "52998224725");
        assert_eq!(cpf.len(), 11);
        assert_eq!(cpf.into_inner(), "52998224725");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serializa_como_string() {
        let cpf = Cpf(String::from("52998224725"));
        assert_eq!(serde_json::to_string(&cpf).unwrap(), "\"52998224725\"");
    }
}
