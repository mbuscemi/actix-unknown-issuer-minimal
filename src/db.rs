use std::io::BufReader;
use std::fs::File;
use std::sync::Arc;
use log::warn;
use deadpool_postgres::Pool;
use rustls::{ ResolvesClientCert, SignatureScheme, sign::CertifiedKey };
use tokio_postgres::NoTls;
use tokio_postgres_rustls::MakeRustlsConnect;

use crate::settings::Settings;

pub fn create_pool(settings: &Settings) -> Pool {
    println!("{:?}", settings.pg.get_pg_config());
    println!("{:?}", settings.pg.get_manager_config());
    println!("{:?}", settings.pg.get_pool_config());
    println!("{:?}", settings.db_ca_cert);

    if settings.use_ssl {
        let mut tls_config = rustls::ClientConfig::new();

        // Look up any certs managed by the operating system.
        if settings.use_rustls_root_store {
            tls_config.root_store = match rustls_native_certs::load_native_certs() {
                Ok(store) => store,
                Err((Some(store), err)) => {
                    warn!("could not load all certificates: {}", err);
                    store
                }
                Err((None, err)) => {
                    warn!("cannot access native certificate store: {}", err);
                    tls_config.root_store
                }
            };
        }

        // Add any webpki certs, too, in case the OS is useless.
        if settings.add_webpki_roots {
            tls_config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        }

        if let Some(ca_cert) = &settings.db_ca_cert {
            let cert_file = File::open(&ca_cert).expect("failed to open pem file");
            let mut buf = BufReader::new(cert_file);
            tls_config.root_store.add_pem_file(&mut buf).map_err(|_| {
                warn!("failed to read database root certificate: {}", ca_cert)
            }).unwrap();
        }

        if settings.use_custom_cert_resolver {
            tls_config.client_auth_cert_resolver = Arc::new(AlwaysResolvesClientCert);
        }

        let tls = MakeRustlsConnect::new(tls_config);
        settings.pg.create_pool(tls).unwrap()
    } else {
        settings.pg.create_pool(NoTls).unwrap()
    }
}

struct AlwaysResolvesClientCert;

impl ResolvesClientCert for AlwaysResolvesClientCert {
    fn resolve(&self, _acceptable_issuers: &[&[u8]], _sigschemes: &[SignatureScheme]) -> Option<CertifiedKey> {
        None
    }

    fn has_certs(&self) -> bool {
        true
    }
}