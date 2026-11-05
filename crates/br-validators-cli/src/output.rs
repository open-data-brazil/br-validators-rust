//! Contrato de saída da CLI TypeScript.

use serde_json::{json, Value};

/// Fluxos capturados e código de saída POSIX.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Output {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub exit_code: u8,
}
impl Output {
    /// Constrói um erro de uso.
    pub fn usage(message: impl Into<String>) -> Self {
        Self {
            stderr: vec![message.into()],
            exit_code: 2,
            ..Self::default()
        }
    }
    /// Constrói uma saída de texto com sucesso.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            stdout: vec![text.into()],
            ..Self::default()
        }
    }
    /// Serializa o valor com a indentação da referência.
    pub fn json(value: &Value, exit_code: u8) -> Self {
        match serde_json::to_string_pretty(value) {
            Ok(text) => Self {
                stdout: vec![text],
                exit_code,
                ..Self::default()
            },
            Err(error) => Self::usage(error.to_string()),
        }
    }
}

/// Exibe a validação, com JSON tendo prioridade sobre quiet.
pub fn validation(
    mut result: Value,
    json: bool,
    quiet: bool,
    source: Option<&str>,
    details: Vec<String>,
) -> Output {
    let valid = result["ok"] == true;
    let code = u8::from(!valid);
    if valid {
        if let Some(source) = source {
            result["source"] = source.into();
        }
    }
    if json {
        return Output::json(&result, code);
    }
    let mut output = Output {
        exit_code: code,
        ..Output::default()
    };
    if quiet {
        return output;
    }
    if valid {
        output.stdout.push(format!(
            "valid: yes ({})",
            result["format"].as_str().unwrap_or_default()
        ));
        output.stdout.push(format!(
            "value: {}",
            result["value"].as_str().unwrap_or_default()
        ));
        output.stdout.extend(details);
        if let Some(source) = source {
            output.stdout.push(format!("source: {source}"));
        }
    } else {
        output.stderr = vec![
            "valid: no".into(),
            format!("code: {}", result["code"].as_str().unwrap_or_default()),
            format!(
                "message: {}",
                result["message"].as_str().unwrap_or_default()
            ),
        ];
    }
    output
}
/// Exibe o resultado de máscara ou conversão.
pub fn format(result: Value, json: bool, quiet: bool) -> Output {
    let valid = result["ok"] == true;
    let code = u8::from(!valid);
    if json {
        return Output::json(&result, code);
    }
    if quiet {
        return Output {
            exit_code: code,
            ..Output::default()
        };
    }
    if valid {
        Output::text(result["formatted"].as_str().unwrap_or_default())
    } else {
        Output {
            stderr: vec![
                format!("code: {}", result["code"].as_str().unwrap_or_default()),
                format!(
                    "message: {}",
                    result["message"].as_str().unwrap_or_default()
                ),
            ],
            exit_code: code,
            ..Output::default()
        }
    }
}
/// A ação strip ignora quiet na referência.
pub fn strip(value: &str, json: bool) -> Output {
    if json {
        Output::json(&json!({"stripped": value}), 0)
    } else {
        Output::text(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn respeita_json_quiet_e_codigos() {
        let valid = json!({"ok": true,"value":"12345678909","format":"numeric"});
        assert!(validation(valid.clone(), false, true, None, vec![])
            .stdout
            .is_empty());
        let result = validation(
            valid,
            true,
            true,
            Some(br_validators::cpf::CPF_OFFICIAL_SOURCE_URL),
            vec![],
        );
        assert_eq!(result.exit_code, 0);
        assert_eq!(
            serde_json::from_str::<Value>(&result.stdout[0]).unwrap()["source"],
            br_validators::cpf::CPF_OFFICIAL_SOURCE_URL
        );
        let error = json!({"ok":false,"code":"EMPTY_INPUT","message":"CPF input is empty"});
        assert_eq!(
            validation(error, false, false, None, vec![]).stderr,
            vec![
                "valid: no",
                "code: EMPTY_INPUT",
                "message: CPF input is empty"
            ]
        );
        assert_eq!(Output::usage("Missing CPF value.").exit_code, 2);
        assert_eq!(strip("123", false).stdout, vec!["123"]);
        assert_eq!(
            format(json!({"ok":true,"formatted":"123"}), false, false).stdout,
            vec!["123"]
        );
    }
}
