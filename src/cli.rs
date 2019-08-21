//! Contains function definitions for handling CLI interactions.

/// Parses the command-line arguments passed to the program, returning a
/// collection of matches.
pub fn get_arguments<'a>() -> clap::ArgMatches<'a> {
    use clap:: {
        crate_authors,
        crate_description,
        crate_name,
        crate_version
    };
    let argument_parser = clap::App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .help_message("Displays help and usage information.")
        .version(crate_version!())
        .version_message("Displays version information")
        .arg(clap::Arg::with_name("listen")
             .default_value("localhost")
             .env("TWS_LISTEN")
             .help("Specifies the address to listen on.")
             .long("--listen")
             .short("-L")
             .value_name("IP")
        )
        .arg(clap::Arg::with_name("log_file")
             .default_value("/var/log/tws.log")
             .env("TWS_LOG_FILE")
             .help("Specifies the log file to write server events to.")
             .long("--log-file")
             .short("-f")
             .value_name("FILE")
        )
        .arg(clap::Arg::with_name("log_level")
             .default_value("info")
             .env("TWS_LOG_LEVEL")
             .help("Specifies the logging level when writing server events.")
             .long("--log-level")
             .possible_values(&[
                 "disabled",
                 "error",
                 "warning",
                 "info",
                 "debug"
             ])
             .short("-l")
             .value_name("LVL")
        )
        .arg(clap::Arg::with_name("log_mode")
             .default_value("append")
             .env("TWS_LOG_MODE")
             .help("Specifies whether to append to, or overwite, the specified log file.")
             .long("--log-mode")
             .short("-M")
             .value_name("MODE")
        )
        .arg(clap::Arg::with_name("path")
             .default_value(".")
             .env("TWS_PATH")
             .help("Specifies the path to serve.")
             .long("--path")
             .short("-p")
             .value_name("PATH")
        )
        .arg(clap::Arg::with_name("port")
             .default_value("8000")
             .env("TWS_PORT")
             .help("Specifies the port to bind to.")
             .long("--port")
             .short("-P")
             .validator( | val_str | {
                 match val_str.parse::<u16>() {
                     Ok(val) if val > 0 => Ok(()),
                     _ => Err(String::from("Specified port is not a positive integer value."))
                 }
             })
             .value_name("INT")
        )
        .settings(
            &[
                clap::AppSettings::ColoredHelp,
            ]
        );
    argument_parser.get_matches()
}
