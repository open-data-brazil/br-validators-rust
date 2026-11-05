use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_br-validators"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn validadores_cli_reproduzem_o_oraculo_e_os_codigos_de_saida() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../br-validators/tests/parity");
    for (command, file) in [
        ("cpf", "cpf"),
        ("cnpj", "cnpj"),
        ("cep", "cep"),
        ("telefone", "telefone"),
        ("cnh", "cnh"),
        ("renavam", "renavam"),
        ("titulo-eleitor", "titulo-eleitor"),
        ("processo-judicial", "processo-judicial"),
        ("rg", "rg"),
        ("nfe-chave", "nfe-chave"),
        ("placa", "placa"),
        ("pis-pasep", "pis-pasep"),
        ("cnis", "cnis"),
        ("pix", "pix"),
        ("brcode", "brcode"),
        ("boleto", "boleto"),
        ("cartao", "cartao-credito"),
        ("ean", "ean"),
        ("ie", "inscricao-estadual"),
    ] {
        let cases: Vec<Value> =
            serde_json::from_slice(&fs::read(root.join(format!("{file}.json"))).unwrap()).unwrap();
        let row = cases
            .iter()
            .find(|row| row["expected"]["ok"] == true)
            .unwrap();
        let mut args = vec![
            command,
            "validate",
            row["input"].as_str().unwrap(),
            "--json",
        ];
        if let Some(uf) = row["opts"]["uf"].as_str() {
            args.extend(["--uf", uf]);
        }
        let out = cli(&args);
        assert_eq!(
            out.status.code(),
            Some(0),
            "{command}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let actual: Value = serde_json::from_slice(&out.stdout).unwrap();
        let mut expected = row["expected"].clone();
        if matches!(
            command,
            "titulo-eleitor" | "processo-judicial" | "nfe-chave"
        ) {
            expected
                .as_object_mut()
                .unwrap()
                .retain(|key, _| matches!(key.as_str(), "ok" | "value" | "format"));
        }
        assert_eq!(actual, expected, "{command}");
        args[2] = "!";
        assert_eq!(cli(&args).status.code(), Some(1), "{command}");
    }
}
#[test]
fn preserva_parse_conversao_metadata_e_aliases() {
    let out = cli(&[
        "nfe",
        "parse",
        "52060433009911002506550120000007800267301615",
        "--json",
    ]);
    assert_eq!(out.status.code(), Some(0));
    let parsed: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(parsed["parsed"]["cUF"], "52");
    assert_eq!(parsed["uf"], "GO");
    assert_eq!(cli(&["placa", "convert", "ABC1234"]).stdout, b"ABC1C34\n");
    assert_eq!(
        cli(&[
            "cnis",
            "validate",
            "10027230888",
            "--issuer",
            "inss",
            "--tipo",
            "nit",
            "--json"
        ])
        .status
        .code(),
        Some(0)
    );
    assert_eq!(
        cli(&["cartao-credito", "validate", "4111111111111111", "--quiet"])
            .status
            .code(),
        Some(0)
    );
    assert_eq!(
        cli(&[
            "ie",
            "validate",
            "P-01100424.3/002",
            "--uf",
            "SP",
            "--quiet"
        ])
        .status
        .code(),
        Some(0)
    );
    assert_eq!(
        cli(&["titulo", "validate", "004356870906", "--quiet"])
            .status
            .code(),
        Some(0)
    );
}
#[test]
fn distingue_uso_invalido_e_erro_de_validacao() {
    let missing = cli(&["cpf", "validate"]);
    assert_eq!(missing.status.code(), Some(2));
    assert_eq!(
        missing.stderr,
        b"Missing CPF value. Pass an argument or use --file.\n"
    );
    assert_eq!(cli(&["rg", "validate", "120300011"]).status.code(), Some(2));
    assert_eq!(
        cli(&["cpf", "validate", "11111111111", "--quiet"]).stdout,
        b""
    );
    assert_eq!(
        cli(&["cpf", "validate", "12345678909", "--json", "--quiet"])
            .status
            .code(),
        Some(0)
    );
}
