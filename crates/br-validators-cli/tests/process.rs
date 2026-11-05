use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

fn command() -> Command {
    Command::cargo_bin("br-validators").unwrap()
}
#[test]
fn snapshots_originais_preservam_o_contrato_json() {
    for (args, snapshot) in [
        (
            vec!["cpf", "validate", "123.456.789-09", "--json"],
            include_str!("snapshots/cpf.json"),
        ),
        (
            vec!["cnpj", "validate", "12ABC34501DE35", "--json"],
            include_str!("snapshots/cnpj.json"),
        ),
        (
            vec!["bancos", "lookup", "001", "--json"],
            include_str!("snapshots/bancos.json"),
        ),
    ] {
        let out = command()
            .args(args)
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        assert_eq!(
            serde_json::from_slice::<Value>(&out).unwrap(),
            serde_json::from_str::<Value>(snapshot).unwrap()
        );
    }
}
#[test]
fn documentos_no_processo_real() {
    for (ty, parity, uf) in [
        ("cpf", "cpf", None),
        ("cnpj", "cnpj", None),
        ("cep", "cep", None),
        ("telefone", "telefone", None),
        ("cnh", "cnh", None),
        ("renavam", "renavam", None),
        ("titulo", "titulo-eleitor", None),
        ("processo-judicial", "processo-judicial", None),
        ("rg", "rg", Some("SP")),
        ("nfe", "nfe-chave", None),
        ("placa", "placa", None),
        ("pis-pasep", "pis-pasep", None),
        ("cnis", "cnis", None),
        ("pix", "pix", None),
        ("brcode", "brcode", None),
        ("boleto", "boleto", None),
        ("cartao", "cartao-credito", None),
        ("ean", "ean", None),
        ("ie", "inscricao-estadual", Some("SP")),
    ] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../br-validators/tests/parity/{parity}.json"));
        let cases: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let case = cases
            .as_array()
            .unwrap()
            .iter()
            .find(|case| {
                case["expected"]["ok"] == true && uf.map_or(true, |uf| case["opts"]["uf"] == uf)
            })
            .unwrap();
        let mut args = vec![ty, "validate", case["input"].as_str().unwrap(), "--json"];
        if let Some(uf) = uf {
            args.extend(["--uf", uf]);
        }
        command()
            .args(args)
            .assert()
            .success()
            .stdout(predicate::str::contains("\"ok\": true"));
        let mut args = vec![ty, "validate", "!", "--quiet"];
        if let Some(uf) = uf {
            args.extend(["--uf", uf]);
        }
        command().args(args).assert().code(1).stdout("").stderr("");
    }
}
#[test]
fn plataforma_e_entrada_por_stdin() {
    for args in [
        vec!["detect", "12345678909"],
        vec!["sanitize", "cpf", " 123.456.789-09 "],
        vec!["mask", "cpf", "12345678909"],
        vec!["compare", "cpf", "12345678909", "123.456.789-09"],
        vec!["diff", "cpf", "12345678909", "123.456.789-09"],
        vec!["generate", "cpf", "--seed", "42"],
        vec!["list"],
    ] {
        command().args(args).assert().success();
    }
    for args in [
        vec!["detect", "!"],
        vec!["sanitize", "cpf", "!"],
        vec!["mask", "cpf", "!"],
        vec!["compare", "cpf", "1", "2"],
        vec!["diff", "cpf", "1", "2"],
    ] {
        command().args(args).assert().code(1);
    }
    command().args(["generate", "unknown"]).assert().code(2);
    command().args(["list", "--unknown"]).assert().code(2);
    command()
        .args(["cpf", "validate", "-", "--json"])
        .write_stdin("123.456.789-09\n")
        .assert()
        .success();
    command()
        .args(["batch", "cpf", "--json"])
        .write_stdin("12345678909\n")
        .assert()
        .success();
    command()
        .args(["batch", "cpf", "--col", "CPF", "--delimiter", ";", "--json"])
        .write_stdin("name;CPF\nAna;12345678909\nBia;11111111111\n")
        .assert()
        .code(1)
        .stdout(predicate::str::contains("\"invalid\": 1"));
    command()
        .args(["batch", "cpf", "--col", "0", "--no-skip-header", "--quiet"])
        .write_stdin("12345678909\n")
        .assert()
        .success()
        .stdout("");
    command()
        .args(["batch", "cpf"])
        .write_stdin("")
        .assert()
        .code(2);
    command()
        .args([
            "cpf",
            "validate",
            "--file",
            "/missing-br-validators-cli-input",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Cannot read file:"));
}
#[test]
fn dados_no_processo_real() {
    for (cmd, code) in [
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
        ("iss-municipal", "3550308"),
    ] {
        command()
            .args([cmd, "lookup", code, "--json"])
            .assert()
            .success();
        command().args([cmd, "lookup", "!"]).assert().failure();
    }
    for args in [
        vec!["irpf", "tabela"],
        vec!["inss", "tabela"],
        vec!["cst", "lookup", "00", "--tax", "icms"],
        vec!["data", "catalog"],
        vec!["feriados", "list", "--year", "2025"],
    ] {
        command().args(args).assert().success();
    }
    for args in [
        vec!["irpf", "calc", "-1"],
        vec!["inss", "calc", "-1"],
        vec!["cst", "lookup", "00", "--tax", "other"],
        vec!["data", "unknown"],
        vec!["feriados", "unknown"],
    ] {
        command().args(args).assert().failure();
    }
    let ptax = br_validators::ptax::get_ptax_list()
        .iter()
        .find(|row| row.moeda == "USD")
        .unwrap();
    command()
        .args(["ptax", "lookup", "USD", "--data", &ptax.data])
        .assert()
        .success();
    command().args(["ptax", "lookup", "XYZ"]).assert().code(1);
    let selic = br_validators::selic::get_selic_list().first().unwrap();
    command()
        .args(["selic", "--date", &selic.data])
        .assert()
        .success();
    command()
        .args(["selic", "--date", "1900-01-01"])
        .assert()
        .code(1);
    let cep = br_validators::cep::get_cep_faixas().first().unwrap();
    command()
        .args(["cep", "faixa", &cep.prefixo])
        .assert()
        .success();
    command().args(["cep", "faixa", "00000"]).assert().code(1);
}
