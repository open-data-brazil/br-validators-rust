//! Geração de valores sintéticos; count é a extensão prevista no plano Rust.

use super::*;
use br::generate::{GeneratableDocumentType, GenerateOptions};

pub(super) fn run(ty: &str, args: &Args, opts: &PlatformOptions) -> Output {
    let Ok(ty) = ty.parse::<GeneratableDocumentType>() else {
        return Output::usage(format!("Unsupported generate type: {ty}"));
    };
    if args.count == 0 {
        return Output::usage("Count must be greater than zero.");
    }
    let options = GenerateOptions {
        masked: args.masked,
        stripped: args.stripped,
        seed: args.seed,
        format: args.format.as_deref().and_then(|value| value.parse().ok()),
        uf: opts.uf,
        brand: args.brand.as_deref().and_then(|brand| brand.parse().ok()),
        ..GenerateOptions::default()
    };
    let mut values = Vec::new();
    for index in 0..args.count {
        let options = GenerateOptions {
            seed: options.seed.map(|seed| seed.wrapping_add(index as u64)),
            ..options.clone()
        };
        match br::generate(ty, &options) {
            Ok(value) => values.push(value),
            Err(error) => return Output::usage(error.to_string()),
        }
    }
    if args.json {
        return if args.count == 1 {
            Output::json(&json!({"ok":true,"value":values[0]}), 0)
        } else {
            Output::json(&json!({"ok":true,"values":values}), 0)
        };
    }
    Output {
        stdout: if args.quiet { vec![] } else { values },
        ..Output::default()
    }
}
