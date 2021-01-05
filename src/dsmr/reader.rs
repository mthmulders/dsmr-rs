use super::settings;

use serialport;
use std::time::Duration;

pub fn connect_to_meter(serial_settings: settings::SerialSettings) {
    log::info!(
        "Connecting to {} using baud rate {}",
        &serial_settings.port,
        &serial_settings.baud_rate
    );

    let _port = serialport::new(&serial_settings.port, serial_settings.baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");
}
