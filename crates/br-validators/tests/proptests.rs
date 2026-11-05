use br_validators::{
    detect, format_document, mask, sanitize, validate_boleto, validate_brcode,
    validate_cartao_credito, validate_cep, validate_cnh, validate_cnpj, validate_cpf,
    validate_document, validate_ean, validate_ie_produtor_rural, validate_inscricao_estadual,
    validate_nfe_chave, validate_pis_pasep, validate_pix_key, validate_placa,
    validate_processo_judicial, validate_renavam, validate_rg, validate_telefone,
    validate_titulo_eleitor, DocumentType, PlatformOptions, Uf,
};
use proptest::prelude::*;

const SAMPLE_UFS: [Uf; 5] = [Uf::SP, Uf::RJ, Uf::MG, Uf::RS, Uf::BA];

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn validadores_nunca_entram_em_panico_com_string_arbitraria(s in "\\PC*") {
        let _ = validate_cpf(&s);
        let _ = validate_cnpj(&s);
        let _ = validate_cep(&s);
        let _ = validate_telefone(&s);
        let _ = validate_cnh(&s);
        let _ = validate_renavam(&s);
        let _ = validate_titulo_eleitor(&s);
        let _ = validate_processo_judicial(&s);
        let _ = validate_nfe_chave(&s);
        let _ = validate_placa(&s);
        let _ = validate_pis_pasep(&s);
        let _ = br_validators::cnis::validate_nit(&s);
        let _ = validate_pix_key(&s);
        let _ = validate_brcode(&s);
        let _ = validate_boleto(&s);
        let _ = validate_cartao_credito(&s);
        let _ = validate_ean(&s);
        let _ = validate_ie_produtor_rural(&s, Uf::SP);

        for uf in SAMPLE_UFS {
            let _ = validate_rg(&s, uf);
            let _ = validate_inscricao_estadual(&s, uf);
        }
    }

    #[test]
    fn plataforma_nunca_entra_em_panico_com_string_arbitraria(s in "\\PC*") {
        let opts = PlatformOptions { uf: Some(Uf::SP) };
        let _ = detect(&s, &opts);

        for doc_type in DocumentType::ALL {
            let _ = validate_document(&s, doc_type, &opts);
            let _ = format_document(doc_type, &s);
            let _ = mask(&s, doc_type, &opts);
            let _ = sanitize(&s, doc_type, &opts);
        }
    }
}

#[cfg(feature = "generate")]
mod generator_properties {
    use super::*;
    use br_validators::generate::{
        generate, GeneratableDocumentType, GenerateFormat, GenerateOptions,
    };

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(30))]

        #[test]
        fn gerador_produz_documentos_que_validam(seed in any::<u64>()) {
            for doc_type in GeneratableDocumentType::ALL {
                let opts = GenerateOptions {
                    format: None,
                    masked: false,
                    stripped: true,
                    seed: Some(seed),
                    uf: Some(Uf::SP),
                    brand: None,
                    pix_key: None,
                    merchant_name: None,
                    merchant_city: None,
                    amount: None,
                    txid: None,
                };

                if let Ok(generated) = generate(doc_type, &opts) {
                    assert!(!generated.is_empty());
                    match doc_type {
                        GeneratableDocumentType::Cpf => {
                            assert!(validate_cpf(&generated).is_ok());
                        }
                        GeneratableDocumentType::Cnpj => {
                            assert!(validate_cnpj(&generated).is_ok());
                        }
                        GeneratableDocumentType::Cep => {
                            assert!(validate_cep(&generated).is_ok());
                        }
                        GeneratableDocumentType::Placa => {
                            assert!(validate_placa(&generated).is_ok());
                        }
                        GeneratableDocumentType::PisPasep => {
                            assert!(validate_pis_pasep(&generated).is_ok());
                        }
                        GeneratableDocumentType::Renavam => {
                            assert!(validate_renavam(&generated).is_ok());
                        }
                        GeneratableDocumentType::Cnh => {
                            assert!(validate_cnh(&generated).is_ok());
                        }
                        GeneratableDocumentType::Telefone => {
                            assert!(validate_telefone(&generated).is_ok());
                        }
                        GeneratableDocumentType::CartaoCredito => {
                            assert!(validate_cartao_credito(&generated).is_ok());
                        }
                        GeneratableDocumentType::InscricaoEstadual => {
                            assert!(validate_inscricao_estadual(&generated, Uf::SP).is_ok());
                        }
                        GeneratableDocumentType::InscricaoEstadualProdutorRural => {
                            assert!(validate_ie_produtor_rural(&generated, Uf::SP).is_ok());
                        }
                        GeneratableDocumentType::TituloEleitor => {
                            assert!(validate_titulo_eleitor(&generated).is_ok());
                        }
                        GeneratableDocumentType::Pix => {
                            assert!(validate_pix_key(&generated).is_ok());
                        }
                        GeneratableDocumentType::NfeChave => {
                            assert!(validate_nfe_chave(&generated).is_ok());
                        }
                        GeneratableDocumentType::Brcode => {
                            assert!(validate_brcode(&generated).is_ok());
                        }
                        GeneratableDocumentType::Boleto | GeneratableDocumentType::BoletoArrecadacao => {
                            assert!(validate_boleto(&generated).is_ok());
                        }
                    }
                }
            }
        }

        #[test]
        fn format_e_strip_sao_inversas(seed in any::<u64>()) {
            let opts = GenerateOptions {
                format: Some(GenerateFormat::Numeric),
                masked: false,
                stripped: true,
                seed: Some(seed),
                uf: Some(Uf::SP),
                brand: None,
                pix_key: None,
                merchant_name: None,
                merchant_city: None,
                amount: None,
                txid: None,
            };

            // CPF
            if let Ok(raw) = generate(GeneratableDocumentType::Cpf, &opts) {
                if let Ok(formatted) = br_validators::format_cpf(&raw) {
                    assert_eq!(br_validators::strip_cpf(&formatted), raw);
                }
            }

            // CNPJ
            if let Ok(raw) = generate(GeneratableDocumentType::Cnpj, &opts) {
                if let Ok(formatted) = br_validators::format_cnpj(&raw) {
                    assert_eq!(br_validators::strip_cnpj(&formatted), raw);
                }
            }

            // CEP
            if let Ok(raw) = generate(GeneratableDocumentType::Cep, &opts) {
                if let Ok(formatted) = br_validators::format_cep(&raw) {
                    assert_eq!(br_validators::strip_cep(&formatted), raw);
                }
            }
        }
    }
}
