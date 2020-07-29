extern crate glob;
use slog::{Drain, o, info};
use chrono::Local;

use std::path::{Path};
use fs::BasePath;

mod fs;



fn main() {

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = std::sync::Mutex::new(drain).fuse();
    
    let root_logger = slog::Logger::root(drain, o!());
    info!(root_logger, "Application started"; "started_at" => format!("{}", Local::now().to_rfc3339()));

    let mut dirs = fs::DirsMapper::new(&root_logger);
    dirs.add_mapping("tmp", &Path::new("/tmp"));

    let bp: &BasePath = dirs.get("tmp").unwrap();
    for item in bp.list() {
        println!("{}", item);
    }

    let pth = Path::new("model.lock");
    println!("{:?}", bp.get(pth));
}