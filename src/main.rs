mod app_config;
mod fs;
mod handlers;
mod cli;

use crate::app_config::{AppConfig, AppState};

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

use log::info;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();


    let config = AppConfig::from_env().unwrap();
    info!(
        "Starting server at http://{}:{}  started at {}",
        config.server.host,
        config.server.port,
        Local::now().to_rfc3339()
    );

    HttpServer::new(move || {
        // https://actix.rs/docs/url-dispatch/
        App::new()
            .data(AppState { cfg: config.clone() })
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(handlers::index_handler))
            .service(
                web::scope("/api/v1").service(
                    web::scope("/map")
                        .route("/", web::get().to(handlers::list_mappings_handler))
                        .route("/{mapping}/list", web::get().to(handlers::list_files_handler))
                        .route("/{mapping}/tree", web::get().to(handlers::tree_files_handler))
                        .route("/{mapping}/file", web::get().to(handlers::get_file_handler))
                        .route("/{mapping}/download", web::get().to(handlers::download_file_handler))
                        .route("/{mapping}/path/{path:.*}", web::get().to(handlers::get_file_by_path_handler))
                )
            )
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
