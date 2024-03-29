//! Contains function definitions useful for logging.

/// Sets-up logging for the program.
pub fn setup(
    log_file: &str,
    log_level: &str,
    log_mode: &str
) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(
                format_args!(
                    "[{}] [{}] [{}] [{}] {}",
                    record.level(),
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    std::process::id(),
                    record.target(),
                    message
                )
            )
        })
        .level(match log_level {
            "disabled" => log::LevelFilter::Off,
            "error"    => log::LevelFilter::Error,
            "warn"     => log::LevelFilter::Warn,
            "info"     => log::LevelFilter::Info,
            "debug"    => log::LevelFilter::Debug,
            _          => log::LevelFilter::Trace,
        })
        .chain(std::fs::OpenOptions::new()
               .write(true)
               .create(true)
               .append(match log_mode {
                   "append" => true,
                   _        => false
               })
               .truncate(match log_mode {
                   "append" => false,
                   _        => true
               })
               .open(log_file)?
        )
        .apply()?;
    return Ok(());
}
