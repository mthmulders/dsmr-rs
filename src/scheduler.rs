use std::{thread, time};

use crate::dsmr;

pub fn main_loop(
    api_settings: dsmr::settings::HostSettings,
    serial_settings: dsmr::settings::SerialSettings,
    read_interval: f64,
) {
    const FAILURE_THRESHOLD: i8 = 20;

    let interval = time::Duration::from_millis((read_interval * 1_000.0).round() as u64);
    let mut consumer = dsmr::sender::DelegatingConsumer::new(&api_settings.hosts);
    let mut failure_count: i8 = 0;

    loop {
        let result = dsmr::reader::connect_to_meter(&serial_settings);
        if result.is_ok() {
            let port = result.unwrap();
            dsmr::reader::read_from_serial_port(port, &mut consumer);
            failure_count = 0;
        } else {
            log::info!(
                "failed to connect to {}",
                &serial_settings.port,
            );
            failure_count += 1;
        }

        if failure_count >= FAILURE_THRESHOLD {
            log::error!(
                "failed to connect to {} for {} times, exiting...",
                &serial_settings.port,
                FAILURE_THRESHOLD,
            );
            return;
        }

        // Close a port following the Resource Acquisition Is Initialization (RAII) paradigm
        // by explicitly dropping the reference.

        thread::sleep(interval);
    }
}
