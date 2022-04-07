#![feature(backtrace)]
#![feature(option_result_contains)]
#![cfg_attr(feature = "integration-test", feature(exit_status_error))]
extern crate core;

use crate::configs::{CORSConfig, Opt};
use crate::state::State;
use crate::utils::{IntoAnyhow, LoggedUnwrap};
use actix_cors::Cors;
use actix_session::storage::{RedisSessionStore, SessionStore};
use actix_web::cookie::SameSite;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;

mod configs;
mod routers;
mod session;
mod state;
mod utils;

/// integration test
#[cfg(test)]
mod test {
    #[cfg(all(not(miri), feature = "integration-test"))]
    mod utils;

    #[cfg(all(not(miri), feature = "integration-test"))]
    pub use utils::*;
}

#[cfg_attr(not(miri), global_allocator)]
#[cfg_attr(miri, allow(dead_code))]
static GLOBAL_MIMALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

async fn startup<S, C, A, B, D>(
    log_level: &str,
    static_dir: PathBuf,
    state_initialization: Arc<S>,
    cors_initialization: Arc<C>,
    session_store: Arc<A>,
    addr: D,
) -> anyhow::Result<()>
where
    S: 'static + Fn() -> Data<State>,
    C: 'static + Fn() -> Option<CORSConfig>,
    A: 'static + Fn() -> B + Send + Sync,
    B: 'static + SessionStore,
    D: std::net::ToSocketAddrs,
{
    std::env::set_var("TACHYON_LOG", log_level);
    env_logger::init_from_env("TACHYON_LOG");
    let state: Data<State> = state_initialization();
    let cors = cors_initialization();
    let server = {
        let state = state.clone();
        let cors = cors.clone();
        actix_web::HttpServer::new(move || {
            let routers = routers::routers(&static_dir);
            let cors = cors
                .as_ref()
                .map(|x| x.middleware())
                .unwrap_or_else(Cors::default);
            actix_web::App::new()
                .wrap(
                    actix_web::middleware::ErrorHandlers::new()
                        .handler(StatusCode::NOT_FOUND, routers::add_error_header)
                        .handler(StatusCode::UNAUTHORIZED, routers::add_error_header)
                        .handler(StatusCode::INTERNAL_SERVER_ERROR, routers::add_error_header)
                        .handler(StatusCode::BAD_REQUEST, routers::add_error_header),
                )
                .wrap(actix_web::middleware::Compress::default())
                // enable logger
                .wrap(actix_web::middleware::Logger::default())
                // cookie session middleware
                .wrap(
                    actix_session::SessionMiddleware::builder(session_store(), state.key.clone())
                        .cookie_name("tachyon_id".to_string())
                        .cookie_http_only(true)
                        .cookie_same_site(SameSite::Strict)
                        .build(),
                )
                .wrap(cors)
                .app_data(state.clone())
                .service(routers)
        })
    };

    server
        .bind(addr)
        .anyhow()
        .logged_unwrap()
        .run()
        .await
        .anyhow()
        .logged_unwrap();

    match state.kv_db.flush_async().await {
        Ok(x) => log::info!("sled finalized with {} bytes flushed", x),
        Err(e) => log::error!("sled flush error {}", e),
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();
    log::info!("loading configs from {:?}", opt.config_file);
    let configs = Data::new(opt.parse_configs().await.logged_unwrap());
    let state = Data::new(State::from_configs(&configs).await.logged_unwrap());
    let cors_config = configs.cors.clone();
    let redis_uri = configs.redis_uri.clone();
    let redis = RedisSessionStore::new(&redis_uri).await.logged_unwrap();
    log::info!("starting server at {}", configs.server_addr);
    startup(
        opt.log_level.as_str(),
        configs.static_dir.clone(),
        Arc::new(move || state.clone()),
        Arc::new(move || cors_config.clone()),
        Arc::new(move || redis.clone()),
        configs.server_addr,
    )
    .await
}
