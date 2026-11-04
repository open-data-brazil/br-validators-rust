use std::{collections::BTreeMap, ops::Range};

use crate::framework::{
    encoding::{decode, Hint},
    text::clean,
};

pub(super) struct Campos {
    pub nome: String,
    pub tipo: String,
    pub situacao: String,
    pub municipio: String,
    pub uf: String,
}

// A tabela dBASE III do shapefile usa descritores de 32 bytes e registros de largura fixa.
pub(super) fn ler(bytes: &[u8]) -> Option<BTreeMap<String, Campos>> {
    if bytes.first() != Some(&0x03) {
        return None;
    }
    let registros = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    let cabecalho = u16::from_le_bytes(bytes.get(8..10)?.try_into().ok()?) as usize;
    let largura = u16::from_le_bytes(bytes.get(10..12)?.try_into().ok()?) as usize;
    let fim_campos = cabecalho.checked_sub(1)?;
    if bytes.get(fim_campos) != Some(&0x0d) {
        return None;
    }
    let mut descritores = bytes.get(32..fim_campos)?.chunks_exact(32);
    if !descritores.remainder().is_empty() {
        return None;
    }
    let mut offset = 1; // byte inicial indica registro ativo ou excluído
    let mut colunas = BTreeMap::new();
    for campo in &mut descritores {
        let nome = campo.get(..11)?.split(|&b| b == 0).next()?;
        let largura = usize::from(*campo.get(16)?);
        let coluna = offset..offset + largura;
        colunas.insert(nome, coluna);
        offset += largura;
    }
    if offset != largura {
        return None;
    }

    let codigo = colunas.get(b"cdi_tuaria".as_slice())?;
    let nome = colunas.get(b"nome".as_slice())?;
    let tipo = colunas.get(b"tipo".as_slice())?;
    let situacao = colunas.get(b"situacao".as_slice())?;
    let cidade = colunas.get(b"cidade".as_slice())?;
    let estado = colunas.get(b"estado".as_slice())?;
    let fim = cabecalho.checked_add(registros.checked_mul(largura)?)?;

    let mut tabela = BTreeMap::new();
    for registro in bytes.get(cabecalho..fim)?.chunks_exact(largura) {
        if registro.first() != Some(&b' ') {
            continue;
        }
        let codigo = texto(registro, codigo)?.to_uppercase();
        tabela.entry(codigo).or_insert(Campos {
            nome: texto(registro, nome)?,
            tipo: texto(registro, tipo)?,
            situacao: texto(registro, situacao)?,
            municipio: texto(registro, cidade)?,
            uf: texto(registro, estado)?,
        });
    }
    Some(tabela)
}

fn texto(registro: &[u8], coluna: &Range<usize>) -> Option<String> {
    let bytes = registro.get(coluna.clone())?;
    Some(clean(decode(bytes, Hint::Windows1252).trim_matches('\0')))
}
