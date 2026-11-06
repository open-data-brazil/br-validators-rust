use std::hint::black_box;
use std::time::Instant;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_validators(c: &mut Criterion) {
    c.bench_function("validate_cpf", |b| {
        b.iter(|| {
            let _ = br_validators::validate_cpf(black_box("123.456.789-09"));
        });
    });

    c.bench_function("validate_cnpj_alphanumeric", |b| {
        b.iter(|| {
            let _ = br_validators::validate_cnpj(black_box("12ABC34501DE35"));
        });
    });

    c.bench_function("detect", |b| {
        b.iter(|| {
            let _ =
                br_validators::detect(black_box("123.456.789-09"), black_box(&Default::default()));
        });
    });

    #[cfg(feature = "ncm")]
    {
        let _ = br_validators::ncm::get_ncm_por_codigo("01012100");
        c.bench_function("get_ncm_por_codigo", |b| {
            b.iter(|| {
                let _ = br_validators::ncm::get_ncm_por_codigo(black_box("01012100"));
            });
        });
    }

    #[cfg(feature = "cnaes")]
    {
        let _ = br_validators::cnaes::search_cnaes("cultivo", Some(10));
        c.bench_function("search_cnaes", |b| {
            b.iter(|| {
                let _ =
                    br_validators::cnaes::search_cnaes(black_box("cultivo"), black_box(Some(10)));
            });
        });
    }
}

fn bench_first_load(_c: &mut Criterion) {
    #[cfg(feature = "ncm")]
    {
        let start = Instant::now();
        let _ = br_validators::ncm::get_all_ncm();
        let elapsed = start.elapsed();
        eprintln!("[bench] primeira carga ncm: {:?}", elapsed);
    }

    #[cfg(feature = "cep-faixas")]
    {
        let start = Instant::now();
        let _ = br_validators::cep_faixas::get_cep_faixa_info("01310100");
        let elapsed = start.elapsed();
        eprintln!("[bench] primeira carga cep-faixas: {:?}", elapsed);
    }
}

criterion_group!(benches, bench_validators, bench_first_load);
criterion_main!(benches);
