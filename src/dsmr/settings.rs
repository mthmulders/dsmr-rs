use std::collections::HashMap;
use std::result::Result;

pub struct SerialSettings {
    pub port: String,
    pub baud_rate: u32,
}

fn read_serial_settings(settings: HashMap<String, String>) -> Result<SerialSettings, String> {
    let serial_port = match settings.get("serial_port") {
        Some(value) => value,
        None => return Err("Setting serial_port not defined".to_string()),
    };
    let serial_baudrate = match settings.get("serial_baudrate") {
        Some(value) => match value.parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                return Err("Setting serial_baudrate can not be converted to a number".to_string())
            }
        },
        None => return Err("Setting serial_baudrate not defined".to_string()),
    };

    Ok(SerialSettings {
        port: serial_port.to_string(),
        baud_rate: serial_baudrate,
    })
}

pub fn serial_settings(settings: config::Config) -> Result<SerialSettings, String> {
    log::trace!("reading settings...");
    settings
        .try_into::<HashMap<String, String>>()
        .map_err(|e| e.to_string())
        .and_then(read_serial_settings)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn read_serial_settings_valid_settings() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));
        settings.insert(String::from("serial_baudrate"), String::from("9600"));

        let result = read_serial_settings(settings);

        assert_eq!(result.is_ok(), true);
        let value = result.unwrap();
        assert_eq!(value.port, "/dev/ttyUSB0");
        assert_eq!(value.baud_rate, 9600);
    }

    #[test]
    fn read_serial_settings_without_serial_port() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_baudrate"), String::from("9600"));

        let result = read_serial_settings(settings);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn read_serial_settings_without_serial_baudrate() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));

        let result = read_serial_settings(settings);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn read_serial_settings_with_invalid_serial_baudrate() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));
        settings.insert(String::from("serial_baudrate"), String::from("a"));

        let result = read_serial_settings(settings);

        assert_eq!(result.is_err(), true);
    }
}
