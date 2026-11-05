use br_validators_cli::{args::Args, output::Output, run};
use clap::Parser;
use serde_json::Value;
fn execute(args: &[&str]) -> Output {
    run(
        &Args::try_parse_from(std::iter::once("br-validators").chain(args.iter().copied()))
            .unwrap(),
    )
}
#[test]
fn consultas_de_referencia_encontram_registros_e_rejeitam_ausentes() {
    for (command, code) in [
        ("bancos", "001"),
        ("ibge", "3550308"),
        ("nfe-cuf", "35"),
        ("ddd", "11"),
        ("tse-municipios", "71072"),
        ("natureza-juridica", "2062"),
        ("nbs", "101011100"),
        ("cest", "0100100"),
        ("cnae", "6201501"),
        ("cfop", "5102"),
        ("csosn", "102"),
        ("ncm", "01012100"),
        ("cbo", "212405"),
        ("moedas", "USD"),
        ("paises-bacen", "1058"),
        ("incoterms", "FOB"),
        ("portos", "BR004"),
        ("aeroportos", "GRU"),
    ] {
        let output = execute(&[command, "lookup", code, "--json"]);
        assert_eq!(output.exit_code, 0, "{command}: {output:?}");
        let value: Value = serde_json::from_str(&output.stdout.join("\n")).unwrap();
        assert_eq!(value["ok"], true);
        assert_ne!(execute(&[command, "lookup", "!"]).exit_code, 0, "{command}");
    }
}
#[test]
fn calendario_fiscal_e_catalogo() {
    for args in [
        vec!["data", "catalog", "--json"],
        vec!["ibge", "list", "estados", "--json"],
        vec!["ibge", "list", "municipios", "--uf", "SP", "--limit", "2"],
        vec!["bancos", "list", "--limit", "2"],
        vec!["feriados", "list", "--year", "2025"],
        vec!["irpf", "tabela"],
        vec!["irpf", "calc", "3000,01", "--json"],
        vec!["inss", "calc", "3000,01", "--json"],
        vec!["cst", "lookup", "00", "--tax", "icms"],
        vec!["cst", "search", "tributada", "--tax", "icms"],
        vec!["cst", "validate", "00", "--tax", "icms"],
        vec!["iss-municipal", "lookup", "3550308", "--json"],
        vec!["iss-municipal", "resolve", "SP", "São Paulo", "--json"],
        vec!["iss-municipal", "list", "--uf", "SP", "--limit", "2"],
        vec!["iss-municipal", "search", "São Paulo", "--json"],
        vec!["ncm", "search", "Reprodutores"],
        vec!["ncm", "validate", "01012100"],
    ] {
        let output = execute(&args);
        assert_eq!(output.exit_code, 0, "{args:?}: {output:?}");
    }
    for args in [
        vec!["irpf", "calc", "-1"],
        vec!["inss", "calc", "-1"],
        vec!["irpf", "tabela", "--ano", "1900"],
        vec!["inss", "tabela", "--ano", "1900"],
        vec!["iss-municipal", "lookup", "0000000"],
    ] {
        assert_eq!(execute(&args).exit_code, 1, "{args:?}");
    }
    let date = br_validators::ptax::get_ptax_list()
        .iter()
        .find(|row| row.moeda == "USD")
        .unwrap()
        .data
        .as_str();
    assert_eq!(
        execute(&["ptax", "lookup", "USD", "--data", date, "--json"]).exit_code,
        0
    );
    assert_eq!(
        execute(&[
            "ptax",
            "historico",
            "USD",
            "--desde",
            date,
            "--ate",
            date,
            "--json"
        ])
        .exit_code,
        0
    );
    let date = br_validators::selic::get_selic_list()
        .first()
        .unwrap()
        .data
        .as_str();
    assert_eq!(execute(&["selic", "--date", date, "--json"]).exit_code, 0);
    assert_eq!(execute(&["selic", "--date", "1900-01-01"]).exit_code, 1);
}
