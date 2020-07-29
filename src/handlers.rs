use crate::app_config::AppState;
use actix_web::{web, HttpResponse, Responder};
use slog::{debug, o};

use serde::Deserialize;
use web::Query;

/**
 App::new()
            .data(AppState { log: log.clone() })
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(handlers::index_handler))
            .route("/api", web::get().to(handlers::api_handler))
            .route("/api/map", web::get().to(handlers::api_map_handler))
            .route("/api/map/{mapping}/list", web::get().to(handlers::list_files_handler))
            .route("/api/map/{mapping}/file", web::get().to(handlers::get_file_handler))
            .route("/api/map/{mapping}/download", web::get().to(handlers::file_download_handler))
            .route("/api/map/{mapping}/path/{path:.*}", web::get().to(handlers::get_path_handler))
*/

pub async fn index_handler(state: web::Data<AppState>) -> impl Responder {
    let sublog = state.log.new(o!("handler" => "index"));

    HttpResponse::Ok().body("Hello world!")
}

pub async fn api_handler(state: web::Data<AppState>) -> impl Responder {
    let sublog = state.log.new(o!("handler" => "api"));

    HttpResponse::Ok().body("Hello world!")
}

pub async fn list_mappings_handler(state: web::Data<AppState>) -> impl Responder {
    let sublog = state.log.new(o!("handler" => "api_map"));

    HttpResponse::Ok().body("Hello world!")
}

pub async fn list_files_handler(
    state: web::Data<AppState>,
    path: web::Path<MappingPath>,
) -> impl Responder {
    let sublog = state.log.new(o!(
        "handler" => "list_files", 
        "mapping" => path.mapping.clone()));
    debug!(sublog, "Listing files");
    HttpResponse::Ok().body("Hello world from list files!")
}

// OPTION for query:  q: web::Query<HashMap<String, String>>
pub async fn get_file_handler(
    state: web::Data<AppState>,
    q: Query<GetFileQuery>,
    path: web::Path<MappingPath>,
) -> impl Responder {
    let sublog = state
        .log
        .new(o!("handler" => "get_file", "file" => q.file.clone()));
    debug!(sublog, "Geting file");
    HttpResponse::Ok().body("Hello world from get file!")
}

pub async fn download_file_handler(
    state: web::Data<AppState>,
    q: Query<GetFileQuery>,
    path: web::Path<MappingPath>,
) -> impl Responder {
    let sublog = state
        .log
        .new(o!("handler" => "download_file", "file" => q.file.clone()));
    debug!(sublog, "Geting file");
    HttpResponse::Ok().body("Hello world from get file!")
}

pub async fn get_file_by_path_handler(
    state: web::Data<AppState>,
    path: web::Path<MappingPathTail>,
) -> impl Responder {
    let sublog = state
        .log
        .new(o!("handler" => "download_file", "mapping" => path.mapping.clone(), "path" => path.path.clone()));
    debug!(sublog, "Geting file");
    HttpResponse::Ok().body("Hello world from get file!")
}

#[derive(Deserialize)]
pub struct GetFileQuery {
    pub file: Option<String>,
    pub format: Option<String>,
}

#[derive(Deserialize)]
pub struct MappingPath {
    pub mapping: String,
}

#[derive(Deserialize)]
pub struct MappingPathTail {
    pub mapping: String,
    pub path: String,
}
