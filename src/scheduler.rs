use std::{thread, time};

use crate::dsmr;

pub fn main_loop(
    api_settings: dsmr::settings::HostSettings,
    serial_settings: dsmr::settings::SerialSettings,
    read_interval: f64,
) {
    let interval = time::Duration::from_millis((read_interval * 1_000.0).round() as u64);

    loop {
        {
            let port = dsmr::reader::connect_to_meter(&serial_settings);
            let mut consumer = dsmr::sender::DelegatingConsumer::new(&api_settings.hosts);

            dsmr::reader::read_from_serial_port(port, &mut consumer);

            // Close a port following the Resource Acquisition Is Initialization (RAII) paradigm
            // by explicitly dropping the reference.
        }

        thread::sleep(interval);
    }
}
