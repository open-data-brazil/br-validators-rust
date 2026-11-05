//! Fontes e nomes copiados da CLI de referência.

pub(super) fn source(command: &str) -> &'static str {
    match command {
        "cpf" => "https://www.gov.br/receitafederal/pt-br/assuntos/cpf",
        "cnpj" => "https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/perguntas-e-respostas/cnpj/cnpj-alfanumerico.pdf",
        "cep" => "https://www.correios.com.br/atendimento/developers/manuais/manual-api-busca-cep",
        "telefone" => "https://www.gov.br/anatel/pt-br/regulado/numeracao/plano-de-numeracao-brasileiro",
        "cnh" => "https://www.gov.br/transportes/pt-br/assuntos/transito/conteudo-contran/resolucoes/resolucao5112014.pdf",
        "renavam" => "https://www.gov.br/transportes/pt-br/assuntos/transito/arquivos-senatran/portarias/2013/portaria0272013.pdf",
        "titulo-eleitor" => "https://www.tse.jus.br/legislacao/compilada/res/1998/resolucao-no-20-132-de-19-de-marco-de-1998",
        "processo-judicial" => "https://atos.cnj.jus.br/atos/detalhar/119",
        "nfe-chave" => "https://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=ndIjl+iEFdE%3D",
        "placa" => "https://www.gov.br/transportes/pt-br/assuntos/transito/conteudo-contran/resolucoes/resolucao7292018consolidada.pdf",
        "pis-pasep" => "https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf",
        "pix" => "https://www.bcb.gov.br/content/estabilidadefinanceira/pix/Regulamento_Pix/II_ManualdePadroesparaIniciacaodoPix.pdf",
        "brcode" => "https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf",
        "boleto" => "https://cmsarquivos.febraban.org.br/Arquivos/documentos/PDF/Conven%C3%A7%C3%A3o%20da%20Cobran%C3%A7a%20-%2005_02_2021_f.pdf",
        "cartao" => "https://www.iso.org/standard/70484.html",
        "ean" => "https://www.gs1.org/standards/barcodes/ean-upc",
        "cnis" => "https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf",
        _ => "",
    }
}

pub(super) const IE_UFS: &str = "AC, AL, AM, AP, BA, CE, DF, ES, GO, MA, MG, MS, MT, PA, PB, PE, PI, PR, RJ, RN, RO, RR, RS, SC, SE, SP, TO";

pub(super) const RG_UFS: &str = "SP, RJ, MG, PR, RS, SC, BA, AC, AL, AM, AP, DF, ES, GO, MA, MS, MT, PA, PB, CE, PE, PI, RN, RO, RR, SE, TO";
