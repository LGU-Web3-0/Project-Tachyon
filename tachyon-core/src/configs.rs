use actix_cors::Cors;
use actix_web::http;
use clap::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Opt {
    /// Path to the config file
    #[clap(long, short, env = "TACHYON_CONFIG", default_value = "./config.toml")]
    pub config_file: PathBuf,
    #[clap(long, short, env = "TACHYON_LOG", default_value = "info")]
    pub log_level: log::Level,
}

impl Opt {
    pub async fn parse_configs(&self) -> crate::utils::Result<Configs> {
        let file = tokio::fs::read_to_string(&self.config_file).await?;
        toml::from_str(&file).map_err(Into::into)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Configs {
    pub db_uri: String,
    pub redis_uri: String,
    pub server_addr: SocketAddr,
    pub sled_dir: PathBuf,
    pub static_dir: PathBuf,
    pub cors: Option<CORSConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CORSConfig {
    pub origin: String,
    pub methods: Option<Vec<String>>,
    pub allow_headers: Option<Vec<String>>,
    pub expose_headers: Option<Vec<String>>,
    pub max_age: Option<usize>,
    pub wildcard: Option<bool>,
}

impl CORSConfig {
    #[cfg(all(not(miri), test, feature = "integration-test"))]
    pub fn accept_all() -> Self {
        CORSConfig {
            origin: "*".to_string(),
            methods: Some(vec![]),
            allow_headers: Some(vec![]),
            expose_headers: Some(vec![]),
            max_age: None,
            wildcard: Some(true),
        }
    }
    pub fn middleware(&self) -> actix_cors::Cors {
        let mut cors = Cors::default();

        if let Some(age) = self.max_age {
            cors = cors.max_age(age);
        }

        if let Some(true) = self.wildcard {
            cors = cors.send_wildcard();
        }

        if self.origin == "*" {
            cors = cors.allow_any_origin();
        } else {
            cors = cors.allowed_origin(&self.origin);
        }

        cors = match &self.methods {
            None => cors.allow_any_method(),
            Some(m) => cors.allowed_methods(
                m.iter()
                    .filter_map(|x| http::Method::from_bytes(x.as_bytes()).ok()),
            ),
        };

        cors = match &self.allow_headers {
            None => cors.allow_any_header(),
            Some(h) => cors.allowed_headers(h),
        };

        cors = match &self.expose_headers {
            None => cors.expose_any_header(),
            Some(h) => cors.expose_headers(h),
        };

        cors
    }
}
