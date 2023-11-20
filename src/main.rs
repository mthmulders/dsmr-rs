use serialport::SerialPort;

mod dsmr;

fn init_logger(debug_logging: bool) {
    let console_level = if debug_logging {
        simplelog::LevelFilter::Debug
    } else {
        simplelog::LevelFilter::Info
    };
    let file_level = if debug_logging {
        simplelog::LevelFilter::Trace
    } else {
        simplelog::LevelFilter::Info
    };
    let config = simplelog::Config::default();

    if debug_logging {
        let file = std::fs::File::create("dsmr-rs.log").unwrap();
        simplelog::CombinedLogger::init(vec![
            simplelog::TermLogger::new(
                console_level,
                config.clone(),
                simplelog::TerminalMode::Mixed,
                simplelog::ColorChoice::Never,
            ),
            simplelog::WriteLogger::new(file_level, config, file),
        ])
        .unwrap()
    } else {
        simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
            console_level,
            config,
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Never,
        )])
        .unwrap()
    }
}

pub fn main() {
    let builder = config::Config::builder()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `DATALOGGER_DEBUG_LOGGING=1 dsmr-rs` would set the `debug_logging` key
        .add_source(config::Environment::with_prefix("DATALOGGER"));
    let settings = builder.build().unwrap();

    let debug_logging = settings.get_bool("debug_logging").unwrap_or(false);
    init_logger(debug_logging);

    log::info!("dsmr-rs starting...");
    let (serial_settings, api_settings) = dsmr::settings::settings(settings).unwrap();
    let mut consumer = dsmr::sender::DelegatingConsumer::new(api_settings.hosts);
    
    let port: Box<dyn SerialPort> = dsmr::reader::connect_to_meter(serial_settings);

    dsmr::reader::read_from_serial_port(port, &mut consumer);
}
