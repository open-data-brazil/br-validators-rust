#![cfg(feature = "remote")]

use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use br_validators::{
    remote::{Catalog, CatalogDataset, CatalogFile, RemoteClient, RemoteError},
    schema::Agendamento,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestBanco {
    pub codigo: String,
    pub nome: String,
}

fn compute_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[test]
fn remote_client_consulta_catalogo_e_baixa_com_checksum() {
    let bancos = vec![
        TestBanco {
            codigo: "001".to_string(),
            nome: "Banco do Brasil S.A.".to_string(),
        },
        TestBanco {
            codigo: "104".to_string(),
            nome: "Caixa Econômica Federal".to_string(),
        },
    ];
    let bancos_json = serde_json::to_vec(&bancos).expect("json bancos");
    let bancos_sha256 = compute_sha256(&bancos_json);

    let catalog = Catalog {
        version: "0.1.1".to_string(),
        tag: "v0.1.1".to_string(),
        repository: "open-data-brazil/br-validators-rust".to_string(),
        datasets: vec![CatalogDataset {
            id: "bancos".to_string(),
            nome: "Bancos Bacen".to_string(),
            fonte: "Bacen".to_string(),
            agendamento: Agendamento::Diario,
            capturado_em: "2026-11-05".to_string(),
            contagens: std::collections::BTreeMap::from([("bancos".to_string(), 2)]),
            arquivos: vec![CatalogFile {
                path: "bancos.json".to_string(),
                sha256: bancos_sha256,
                bytes: bancos_json.len() as u64,
                release_asset: "bancos--bancos.json".to_string(),
                raw_url: "https://raw.githubusercontent.com/...".to_string(),
            }],
        }],
    };
    let catalog_json = serde_json::to_vec(&catalog).expect("json catalogo");

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let port = listener.local_addr().expect("local addr").port();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let server_thread = thread::spawn(move || {
        listener.set_nonblocking(true).expect("set nonblocking");
        while running_clone.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_nonblocking(false)
                        .expect("stream em modo bloqueante");
                    let mut buf = Vec::new();
                    let mut chunk = [0u8; 1024];
                    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        match stream.read(&mut chunk) {
                            Ok(0) => break,
                            Ok(n) => buf.extend_from_slice(&chunk[..n]),
                            Err(_) => break,
                        }
                    }
                    let req = String::from_utf8_lossy(&buf);
                    let path = req.split_whitespace().nth(1).unwrap_or("");

                    let (status, body) = if path.ends_with("/catalog.json") {
                        ("200 OK", catalog_json.clone())
                    } else if path.ends_with("/bancos--bancos.json") {
                        ("200 OK", bancos_json.clone())
                    } else {
                        ("404 Not Found", b"Not Found".to_vec())
                    };

                    let response = format!(
                        "HTTP/1.1 {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        status,
                        body.len()
                    );
                    let _ = stream.write_all(response.as_bytes());
                    let _ = stream.write_all(&body);
                    let _ = stream.flush();
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(std::time::Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });

    let client = RemoteClient::with_base(format!("http://127.0.0.1:{port}"));
    let cat = client.catalog().expect("catalogo baixado");
    assert_eq!(cat.version, "0.1.1");
    assert_eq!(cat.datasets.len(), 1);

    let downloaded: Vec<TestBanco> = client
        .fetch("bancos", "bancos.json")
        .expect("bancos baixados com checksum ok");
    assert_eq!(downloaded, bancos);

    running.store(false, Ordering::Relaxed);
    let _ = server_thread.join();
}

#[test]
fn remote_client_detecta_divergencia_de_checksum() {
    let catalog = Catalog {
        version: "0.1.1".to_string(),
        tag: "v0.1.1".to_string(),
        repository: "open-data-brazil/br-validators-rust".to_string(),
        datasets: vec![CatalogDataset {
            id: "bancos".to_string(),
            nome: "Bancos Bacen".to_string(),
            fonte: "Bacen".to_string(),
            agendamento: Agendamento::Diario,
            capturado_em: "2026-11-05".to_string(),
            contagens: std::collections::BTreeMap::new(),
            arquivos: vec![CatalogFile {
                path: "bancos.json".to_string(),
                sha256: "0000000000000000000000000000000000000000000000000000000000000000"
                    .to_string(),
                bytes: 10,
                release_asset: "bancos--bancos.json".to_string(),
                raw_url: "https://raw.githubusercontent.com/...".to_string(),
            }],
        }],
    };
    let catalog_json = serde_json::to_vec(&catalog).expect("json catalogo");
    let corrupt_payload = b"[{\"codigo\":\"001\",\"nome\":\"Teste\"}]".to_vec();

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let port = listener.local_addr().expect("local addr").port();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let server_thread = thread::spawn(move || {
        listener.set_nonblocking(true).expect("set nonblocking");
        while running_clone.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_nonblocking(false)
                        .expect("stream em modo bloqueante");
                    let mut buf = Vec::new();
                    let mut chunk = [0u8; 1024];
                    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        match stream.read(&mut chunk) {
                            Ok(0) => break,
                            Ok(n) => buf.extend_from_slice(&chunk[..n]),
                            Err(_) => break,
                        }
                    }
                    let req = String::from_utf8_lossy(&buf);
                    let path = req.split_whitespace().nth(1).unwrap_or("");

                    let (status, body) = if path.ends_with("/catalog.json") {
                        ("200 OK", catalog_json.clone())
                    } else if path.ends_with("/bancos--bancos.json") {
                        ("200 OK", corrupt_payload.clone())
                    } else {
                        ("404 Not Found", b"Not Found".to_vec())
                    };

                    let response = format!(
                        "HTTP/1.1 {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        status,
                        body.len()
                    );
                    let _ = stream.write_all(response.as_bytes());
                    let _ = stream.write_all(&body);
                    let _ = stream.flush();
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(std::time::Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });

    let client = RemoteClient::with_base(format!("http://127.0.0.1:{port}"));
    let res = client.fetch::<TestBanco>("bancos", "bancos.json");
    match res {
        Err(RemoteError::ChecksumMismatch { expected, actual }) => {
            assert_eq!(
                expected,
                "0000000000000000000000000000000000000000000000000000000000000000"
            );
            assert_ne!(actual, expected);
        }
        other => panic!("Esperava ChecksumMismatch, obtive: {other:?}"),
    }

    running.store(false, Ordering::Relaxed);
    let _ = server_thread.join();
}
