//! Leitura de lotes em linhas ou CSV, sem rede.

use super::*;
use std::{
    io::{self, IsTerminal},
    path::Path,
};

pub(super) fn run(ty: DocumentType, args: &Args, opts: &PlatformOptions) -> Output {
    if args.file.is_none() && io::stdin().is_terminal() {
        return Output::usage(
            "Missing input. Pass --file <path> or pipe one value per line on stdin.",
        );
    }
    let raw = match input::read_file(args.file.as_deref().unwrap_or(Path::new("-"))) {
        Ok(value) => value,
        Err(message) => return Output::usage(message),
    };
    let mut values: Vec<String> = if let Some(col) = args.col.as_deref() {
        match br::batch::parse_batch_csv(
            &raw,
            br::batch::ParseBatchCsvOptions {
                col,
                delimiter: args.delimiter.unwrap_or(','),
                skip_header: !args.no_skip_header,
            },
        ) {
            Ok(values) => values,
            Err(error) => return Output::usage(error.to_string()),
        }
    } else {
        raw.split('\n')
            .map(input::trim_input)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect()
    };
    if values.is_empty() {
        return Output::usage("No values to validate.");
    }
    if let Some(limit) = args.limit.filter(|&limit| limit > 0) {
        values.truncate(limit);
    }
    let result = br::batch(values, ty, opts);
    let mut out = render(&json!(result), u8::from(result.summary.invalid != 0), args);
    if !args.json && !args.quiet {
        out.stdout = vec![
            format!("total: {}", result.summary.total),
            format!("valid: {}", result.summary.valid),
            format!("invalid: {}", result.summary.invalid),
        ];
        out.stdout.extend(
            result
                .valid
                .iter()
                .map(|entry| format!("ok[{}]: {}", entry.index, entry.value)),
        );
        out.stderr.extend(
            result
                .invalid
                .iter()
                .map(|entry| format!("fail[{}]: {} — {}", entry.index, entry.code, entry.message)),
        );
    }
    out
}
