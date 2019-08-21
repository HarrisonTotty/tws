//! A tiny web server written in Rust.

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;

pub mod cli;
pub mod logging;

use rocket_contrib::serve::StaticFiles;

/// The entry point of the program.
pub fn main() {
    // Parse CLI arguments.
    let args = cli::get_arguments();

    // Set-up logging.
    match logging::setup(
        args.value_of("log_file").unwrap(),
        args.value_of("log_level").unwrap(),
        args.value_of("log_mode").unwrap()
    ) {
        Ok(_)  => debug!("Initialized logging subsystem."),
        Err(e) => { eprintln!("Error: Unable to initialize logging subsystem - {}", e); std::process::exit(1); }
    }

    // Configure the web server.
    std::env::set_var("ROCKET_CLI_COLORS","off");
    let config = rocket::config::Config::build(rocket::config::Environment::Production)
        .address(args.value_of("listen").unwrap())
        .port(args.value_of("port").unwrap().parse::<u16>().unwrap())
        .log_level(match args.value_of("log_level").unwrap() {
            "disabled" => rocket::config::LoggingLevel::Off,
            "error"    => rocket::config::LoggingLevel::Critical,
            "warning"  => rocket::config::LoggingLevel::Critical,
            "info"     => rocket::config::LoggingLevel::Normal,
            _          => rocket::config::LoggingLevel::Debug
        })
        .finalize()
        .unwrap();
    
    // Launch the rocket!
    rocket::custom(config)
        .mount("/", StaticFiles::from(args.value_of("path").unwrap()))
        .launch();
}
