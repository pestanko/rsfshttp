extern crate glob;
use slog::{Drain, o, info};
use std::path::{PathBuf, Path};

mod fs;



fn main() {

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = std::sync::Mutex::new(drain).fuse();
    
    let root_logger = slog::Logger::root(drain, o!());

    let bp = fs::BasePath::new(&PathBuf::from("/tmp"), &root_logger);

    for item in bp.list() {
        println!("{}", item);
    }
}
