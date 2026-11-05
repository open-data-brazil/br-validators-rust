use br_validators_cli::{args::Args, run};
use clap::Parser;
use serde_json::Value;

fn execute(args: &[&str]) -> br_validators_cli::output::Output {
    run(
        &Args::try_parse_from(std::iter::once("br-validators").chain(args.iter().copied()))
            .unwrap(),
    )
}
#[test]
fn plataforma_preserva_contratos_e_codigos() {
    for args in [
        vec!["detect", "12345678909"],
        vec!["sanitize", "cpf", " 123.456.789-09 "],
        vec!["mask", "cpf", "12345678909"],
        vec!["compare", "cpf", "123.456.789-09", "12345678909"],
        vec!["diff", "cpf", "123.456.789-09", "12345678909"],
        vec!["generate", "cpf", "--seed", "42"],
    ] {
        let mut json = args.clone();
        json.push("--json");
        let output = execute(&json);
        assert_eq!(output.exit_code, 0, "{json:?}: {output:?}");
        let value: Value = serde_json::from_str(&output.stdout.join("\n")).unwrap();
        assert!(value.is_object());
        let mut quiet = args;
        quiet.push("--quiet");
        assert!(execute(&quiet).stdout.is_empty());
    }
    for args in [
        vec!["detect", "!"],
        vec!["sanitize", "cpf", "!"],
        vec!["mask", "cpf", "!"],
        vec!["compare", "cpf", "1", "2"],
        vec!["diff", "cpf", "1", "2"],
    ] {
        assert_eq!(execute(&args).exit_code, 1, "{args:?}");
    }
    for command in ["sanitize", "mask", "compare", "diff", "generate", "batch"] {
        assert_eq!(execute(&[command, "unknown"]).exit_code, 2);
    }
    assert_eq!(execute(&["list"]).stdout.len(), 17);
    assert_eq!(
        execute(&["generate", "cpf", "--count", "2", "--seed", "42"])
            .stdout
            .len(),
        2
    );
}
