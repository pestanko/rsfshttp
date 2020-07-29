extern crate glob;
mod app_config;
mod fs;
mod handlers;

use crate::app_config::{AppConfig, AppState};
use crate::handlers::*;

use chrono::Local;
use slog::info;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = AppConfig::from_env().unwrap();
    let log = AppConfig::configure_log();
    info!(
        log,
        "Starting server at http://{}:{}  started at {}",
        config.server.host,
        config.server.port,
        Local::now().to_rfc3339()
    );

    HttpServer::new(move || {
        App::new()
            .data(AppState { log: log.clone() })
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index_handler))
            .route("/list", web::get().to(list_files_handler))
            .route("/file", web::get().to(get_file_handler))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
