extern crate simplelog;

fn init_logger(debug_logging: bool) {
    let file = std::fs::File::create("dsmr-rs.log").unwrap();
    let level = if debug_logging {
        simplelog::LevelFilter::Debug
    } else {
        simplelog::LevelFilter::Info
    };
    let config = &simplelog::Config::default();

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(level, config.clone(), simplelog::TerminalMode::Mixed),
        simplelog::WriteLogger::new(level, config.clone(), file),
    ])
    .unwrap()
}

pub fn main() {
    let mut settings = config::Config::default();
    settings
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `DATALOGGER_DEBUG_LOGGING=1 dsmr-rs` would set the `debug_logging` key
        .merge(config::Environment::with_prefix("DATALOGGER"))
        .unwrap();

    let debug_logging = settings.get_bool("debug_logging").unwrap_or(false);
    init_logger(debug_logging);

    log::info!("dsmr-rs starting...");
}
