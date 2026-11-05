use std::{fs, io::Write, path::Path, process::Command};

use anyhow::{Context, Result};
use flate2::{write::GzEncoder, Compression};

pub struct FeatureRow {
    pub name: String,
    pub raw_bytes: u64,
    pub gzip_bytes: u64,
    pub notes: String,
}

pub fn run(repo_root: &Path, out_dir: &Path) -> Result<()> {
    let br_validators_path = repo_root.join("crates").join("br-validators");
    let temp_root = std::env::temp_dir().join(format!("br-measure-{}", std::process::id()));
    fs::create_dir_all(&temp_root)?;

    let target_dir = temp_root.join("target");
    let src_dir = temp_root.join("src");
    fs::create_dir_all(&src_dir)?;

    let features_to_measure: Vec<(&str, Option<&str>, &str, &str)> = vec![
        (
            "default",
            None,
            "validator only (zero dependencies)",
            "let _ = br_validators::validate_cpf(\"12345678909\");",
        ),
        (
            "generate",
            Some("generate"),
            "synthetic fixtures generator",
            "let _ = br_validators::generate(br_validators::generate::GeneratableDocumentType::Cpf, &br_validators::generate::GenerateOptions::default());",
        ),
        (
            "remote",
            Some("remote"),
            "dynamic release client",
            "let _ = br_validators::remote::RemoteClient::latest();",
        ),
        (
            "fiscal",
            Some("fiscal"),
            "fiscal domain group",
            "let _ = br_validators::data::ncm::get_ncm_por_codigo(\"01012100\");",
        ),
        (
            "trade",
            Some("trade"),
            "foreign trade domain group",
            "let _ = br_validators::data::moedas::get_moeda_por_codigo(\"USD\");",
        ),
        (
            "finance",
            Some("finance"),
            "banking and monetary group",
            "let _ = br_validators::data::bancos::get_banco_por_codigo(\"001\");",
        ),
        (
            "payroll",
            Some("payroll"),
            "labor and tax brackets group",
            "let _ = br_validators::data::irpf::get_irpf_tabela_progressiva(None);",
        ),
        (
            "logistics",
            Some("logistics"),
            "ports, airports and fuels group",
            "let _ = br_validators::data::aeroportos::get_aeroportos();",
        ),
        (
            "geo",
            Some("geo"),
            "geographic data and holidays",
            "let _ = br_validators::data::ibge::get_estados();",
        ),
        (
            "gov",
            Some("gov"),
            "transparency and public procurement",
            "let _ = br_validators::data::pncp_reference::get_pncp_modalidades();",
        ),
        (
            "all-data",
            Some("all-data"),
            "all embedded datasets",
            "let _ = br_validators::data_catalog::get_data_catalog();",
        ),
        (
            "full",
            Some("full"),
            "all features including remote and generate",
            "let _ = br_validators::data_catalog::get_data_catalog();",
        ),
        (
            "aeroportos",
            Some("aeroportos"),
            "includes embed",
            "let _ = br_validators::data::aeroportos::get_aeroportos();",
        ),
        (
            "anp-combustiveis",
            Some("anp-combustiveis"),
            "includes embed",
            "let _ = br_validators::data::anp_combustiveis::get_all_anp_precos_medios();",
        ),
        (
            "bancos",
            Some("bancos"),
            "includes embed",
            "let _ = br_validators::data::bancos::get_banco_por_codigo(\"001\");",
        ),
        (
            "cbo",
            Some("cbo"),
            "includes embed",
            "let _ = br_validators::data::cbo::get_cbo_por_codigo(\"212405\");",
        ),
        (
            "cep-faixas",
            Some("cep-faixas"),
            "includes embed",
            "let _ = br_validators::data::cep_faixas::get_cep_faixa_info(\"01310\");",
        ),
        (
            "cest",
            Some("cest"),
            "includes embed",
            "let _ = br_validators::data::cest::get_cest_por_codigo(\"0302100\");",
        ),
        (
            "cfop",
            Some("cfop"),
            "includes embed",
            "let _ = br_validators::data::cfop::get_cfop_por_codigo(\"1102\");",
        ),
        (
            "cnaes",
            Some("cnaes"),
            "includes embed",
            "let _ = br_validators::data::cnaes::get_cnae_por_codigo(\"6201501\");",
        ),
        (
            "cnpj-motivos",
            Some("cnpj-motivos"),
            "includes embed",
            "let _ = br_validators::data::cnpj_motivos::get_motivos_situacao_cadastral();",
        ),
        (
            "csosn",
            Some("csosn"),
            "includes embed",
            "let _ = br_validators::data::csosn::get_all_csosn();",
        ),
        (
            "cst",
            Some("cst"),
            "includes embed",
            "let _ = br_validators::data::cst::get_all_cst_icms();",
        ),
        (
            "esocial",
            Some("esocial"),
            "includes embed",
            "let _ = br_validators::data::esocial::get_esocial_categoria_por_codigo(\"101\");",
        ),
        (
            "feriados",
            Some("feriados"),
            "includes embed",
            "let _ = br_validators::data::feriados::is_feriado_nacional(\"2026-01-01\");",
        ),
        (
            "ibge",
            Some("ibge"),
            "includes embed",
            "let _ = br_validators::data::ibge::get_estados();",
        ),
        (
            "ibpt",
            Some("ibpt"),
            "includes embed",
            "let _ = br_validators::data::ibpt::get_all_ibpt_cargas();",
        ),
        (
            "incoterms",
            Some("incoterms"),
            "includes embed",
            "let _ = br_validators::data::incoterms::get_incoterm_por_codigo(\"FOB\");",
        ),
        (
            "inss",
            Some("inss"),
            "includes embed",
            "let _ = br_validators::data::inss::get_inss_tabela_contribuicao(None);",
        ),
        (
            "irpf",
            Some("irpf"),
            "includes embed",
            "let _ = br_validators::data::irpf::get_irpf_tabela_progressiva(None);",
        ),
        (
            "iss-municipal",
            Some("iss-municipal"),
            "includes embed",
            "let _ = br_validators::data::iss_municipal::get_iss_municipal_por_ibge(3550308);",
        ),
        (
            "lc116",
            Some("lc116"),
            "includes embed",
            "let _ = br_validators::data::lc116::get_lc116_por_codigo(\"1.01\");",
        ),
        (
            "moedas",
            Some("moedas"),
            "includes embed",
            "let _ = br_validators::data::moedas::get_moeda_por_codigo(\"BRL\");",
        ),
        (
            "natureza-juridica",
            Some("natureza-juridica"),
            "includes embed",
            "let _ = br_validators::data::natureza_juridica::get_all_natureza_juridica();",
        ),
        (
            "nbs",
            Some("nbs"),
            "includes embed",
            "let _ = br_validators::data::nbs::get_all_nbs();",
        ),
        (
            "ncm",
            Some("ncm"),
            "includes embed",
            "let _ = br_validators::data::ncm::get_ncm_por_codigo(\"01012100\");",
        ),
        (
            "nfe-cuf",
            Some("nfe-cuf"),
            "includes embed",
            "let _ = br_validators::data::nfe_cuf::get_all_cuf();",
        ),
        (
            "paises-bacen",
            Some("paises-bacen"),
            "includes embed",
            "let _ = br_validators::data::paises_bacen::get_pais_por_codigo_bacen(\"1058\");",
        ),
        (
            "pncp-reference",
            Some("pncp-reference"),
            "includes embed",
            "let _ = br_validators::data::pncp_reference::get_pncp_modalidades();",
        ),
        (
            "portos",
            Some("portos"),
            "includes embed",
            "let _ = br_validators::data::portos::get_porto_por_codigo(\"BRSSZ\");",
        ),
        (
            "ptax",
            Some("ptax"),
            "includes embed",
            "let _ = br_validators::data::ptax::get_ptax_ultimo_dia_util(\"USD\");",
        ),
        (
            "selic",
            Some("selic"),
            "includes embed",
            "let _ = br_validators::data::selic::get_selic_meta();",
        ),
        (
            "simples-nacional",
            Some("simples-nacional"),
            "includes embed",
            "let _ = br_validators::data::simples_nacional::get_all_simples_anexos();",
        ),
        (
            "telefone-ddd",
            Some("telefone-ddd"),
            "includes embed",
            "let _ = br_validators::data::telefone_ddd::get_ddd_municipios();",
        ),
        (
            "transparencia-snapshots",
            Some("transparencia-snapshots"),
            "includes embed",
            "let _ = br_validators::data::transparencia_snapshots::get_transparencia_endpoints();",
        ),
        (
            "tse-municipios",
            Some("tse-municipios"),
            "includes embed",
            "let _ = br_validators::data::tse_municipios::get_mapeamento_tse_ibge();",
        ),
    ];

    println!(
        "Medindo tamanho de binários para {} features...",
        features_to_measure.len()
    );
    let mut rows = Vec::new();

    for (name, feature_flag, notes, expr) in features_to_measure {
        let feature_attr = feature_flag
            .map(|f| format!("features = [\"{f}\"]"))
            .unwrap_or_default();

        let cargo_toml = format!(
            r#"[package]
name = "measure-bin"
version = "0.1.0"
edition = "2021"

[dependencies]
br-validators = {{ path = "{}", default-features = false, {feature_attr} }}
"#,
            br_validators_path.display()
        );

        fs::write(temp_root.join("Cargo.toml"), cargo_toml)?;

        let main_rs = format!(
            r#"fn main() {{
    println!("{{}}", br_validators::VERSION);
    {expr}
}}
"#
        );
        fs::write(src_dir.join("main.rs"), main_rs)?;

        let output = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--target-dir",
                target_dir.to_str().unwrap_or("target"),
            ])
            .current_dir(&temp_root)
            .output()
            .with_context(|| format!("Falha ao compilar binário para feature {name}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Falha na compilação da feature {name}:\n{stderr}");
        }

        let bin_path = target_dir.join("release").join("measure-bin");
        let bytes = fs::read(&bin_path)?;
        let raw_len = bytes.len() as u64;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&bytes)?;
        let gz_bytes = encoder.finish()?;
        let gz_len = gz_bytes.len() as u64;

        rows.push(FeatureRow {
            name: name.to_string(),
            raw_bytes: raw_len,
            gzip_bytes: gz_len,
            notes: notes.to_string(),
        });

        println!(
            "  {name:25} | raw: {:8.1} KB | gzip: {:7.1} KB",
            (raw_len as f64) / 1024.0,
            (gz_len as f64) / 1024.0
        );
    }

    // Ordenar do maior para o menor gzip
    rows.sort_by_key(|a| std::cmp::Reverse(a.gzip_bytes));

    let mut md = String::new();
    md.push_str("# br-validators feature binary sizes\n\n");
    md.push_str("> Measured with `cargo run -p br-data-bot -- measure-features` — release binary per feature, gzip level default.\n");
    md.push_str("> Sizes **include embedded JSON** for reference-data features.\n\n");
    md.push_str("| Feature | Raw (release) | Gzip | Notes |\n");
    md.push_str("|---------|---------------|------|-------|\n");

    for r in &rows {
        md.push_str(&format!(
            "| `{}` | {:.1} KB | {:.1} KB | {} |\n",
            r.name,
            (r.raw_bytes as f64) / 1024.0,
            (r.gzip_bytes as f64) / 1024.0,
            r.notes
        ));
    }

    let dest_dir = repo_root.join(out_dir);
    fs::create_dir_all(&dest_dir)?;
    let dest_file = dest_dir.join("feature-sizes.md");
    fs::write(&dest_file, md)?;

    println!("\nTabela gerada com sucesso em {}", dest_file.display());
    let _ = fs::remove_dir_all(&temp_root);
    Ok(())
}
