use clap::{crate_authors, crate_description, crate_version, App, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("rsfshttp")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Path to a config file other than config.toml in the root of project")
        )
        .subcommands(vec![
            SubCommand::with_name("serve")
                .alias("s")
                .about("Serve the files.")
                .args(&[
                    Arg::with_name("host")
                        .short("H")
                        .long("host")
                        .default_value("127.0.0.1")
                        .help("Interface to bind on"),
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .default_value("8080")
                        .help("Which port to use"),
                    Arg::with_name("mapping")
                        .short("M")
                        .long("mapping")
                        .takes_value(true)
                        .help("Define new mapping"),
                ]),
            SubCommand::with_name("config")
                .about("Print out the current configuration"),
        ])
}