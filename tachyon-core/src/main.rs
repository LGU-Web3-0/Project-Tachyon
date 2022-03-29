#![feature(backtrace)]

use crate::configs::Opt;
use crate::state::State;
use crate::utils::{IntoAnyhow, LoggedUnwrap};
use actix_cors::Cors;
use actix_session::storage::RedisActorSessionStore;
use actix_web::cookie::SameSite;
use actix_web::web::Data;
use clap::Parser;
mod configs;
mod routers;
mod state;
mod utils;

#[global_allocator]
static GLOBAL_MIMALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();
    std::env::set_var("TACHYON_LOG", opt.log_level.as_str());
    env_logger::init_from_env("TACHYON_LOG");
    log::info!("loading configs from {:?}", opt.config_file);
    let configs = Data::new(opt.parse_configs().await.logged_unwrap());
    let state = Data::new(State::from_configs(&configs).await.logged_unwrap());
    log::info!("starting server at {}", configs.server_addr);

    let server = {
        let state = state.clone();
        let configs = configs.clone();
        actix_web::HttpServer::new(move || {
            let cors = configs
                .cors
                .as_ref()
                .map(|x| x.middleware())
                .unwrap_or_else(Cors::default);
            actix_web::App::new()
                // enable logger
                .wrap(actix_web::middleware::Logger::default())
                // cookie session middleware
                .wrap(
                    actix_session::SessionMiddleware::builder(
                        RedisActorSessionStore::new(&configs.redis_uri),
                        state.key.clone(),
                    )
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .build(),
                )
                .wrap(cors)
                .app_data(state.clone())
                .service(routers::routers())
        })
    };

    server
        .bind(configs.server_addr)
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

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        println!("welcome!");
    }
}
