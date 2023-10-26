use std::collections::HashMap;
use std::result::Result;

#[derive(Debug)]
#[derive(PartialEq)]    
pub enum ParityBitSetting {
    None,
    Even,
    Odd
}

pub struct SerialSettings {
    pub port: String,
    pub baud_rate: u32,
    pub parity_bit: ParityBitSetting,
}

pub struct Host {
    pub address: String,
    pub key: String,
}

pub struct HostSettings {
    pub hosts: Vec<Host>,
}

fn read_serial_settings(settings: &HashMap<String, String>) -> Result<SerialSettings, String> {
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
    let x = settings.get("serial_parity");
    let parity_bit = match x {
        Some(value) => match value.as_str() {
            "O" => ParityBitSetting::Odd,
            "E" => ParityBitSetting::Even,
            "N" => ParityBitSetting::None,
            &_ => return Err("Value for party_bit not valid".to_string()),
        }
        None => ParityBitSetting::None,
    };

    Ok(SerialSettings {
        port: serial_port.to_string(),
        baud_rate: serial_baudrate,
        parity_bit: parity_bit
    })
}

fn read_host_settings(settings: &HashMap<String, String>) -> Result<HostSettings, String> {
    let hosts: Vec<&str> = match settings.get("api_hosts") {
        Some(value) => value.split(',').collect(),
        None => return Err("Setting api_hosts not defined".to_string()),
    };
    let keys: Vec<&str> = match settings.get("api_keys") {
        Some(value) => value.split(',').collect(),
        None => return Err("Setting api_hosts not defined".to_string()),
    };

    if hosts.len() != keys.len() {
        let msg = format!(
            "Number of items in api_hosts ({}) is not equal to number of items in api_keys ({})",
            hosts.len(),
            keys.len()
        );
        return Err(msg);
    }

    let result = (0..hosts.len())
        .map(|x| Host {
            address: String::from(hosts[x]),
            key: String::from(keys[x]),
        })
        .collect::<Vec<Host>>();

    Ok(HostSettings { hosts: result })
}

pub fn settings(settings: config::Config) -> Result<(SerialSettings, HostSettings), String> {
    let config_map = settings
        .try_deserialize::<HashMap<String, String>>()
        .map_err(|e| e.to_string())
        .unwrap();

    let serial_settings = read_serial_settings(&config_map);
    let host_settings = read_host_settings(&config_map);

    match (serial_settings, host_settings) {
        (Ok(s), Ok(h)) => Ok((s, h)),
        (Ok(_), Err(msg)) => Err(msg),
        (Err(msg), Ok(_)) => Err(msg),
        (Err(msg1), Err(msg2)) => Err([msg1, msg2].join(" + ")),
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn read_serial_settings_valid_settings() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));
        settings.insert(String::from("serial_baudrate"), String::from("9600"));

        let result = read_serial_settings(&settings);

        assert_eq!(result.is_ok(), true);
        let value = result.unwrap();
        assert_eq!(value.port, "/dev/ttyUSB0");
        assert_eq!(value.baud_rate, 9600);
    }

    #[test]
    fn read_serial_settings_without_serial_port() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_baudrate"), String::from("9600"));

        let result = read_serial_settings(&settings);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn read_serial_settings_without_serial_baudrate() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));

        let result = read_serial_settings(&settings);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn read_serial_settings_with_invalid_serial_baudrate() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));
        settings.insert(String::from("serial_baudrate"), String::from("a"));

        let result = read_serial_settings(&settings);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn read_serial_settings_with_parity_bit_odd() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));
        settings.insert(String::from("serial_baudrate"), String::from("9600"));
        settings.insert(String::from("serial_parity"), String::from("O"));
        
        let result = read_serial_settings(&settings);

        assert_eq!(result.is_ok(), true);
        let value = result.unwrap();
        assert_eq!(value.parity_bit, ParityBitSetting::Odd);
    }

    #[test]
    fn read_serial_settings_with_parity_bit_even() {
        let mut settings = HashMap::new();
        settings.insert(String::from("serial_port"), String::from("/dev/ttyUSB0"));
        settings.insert(String::from("serial_baudrate"), String::from("9600"));
        settings.insert(String::from("serial_parity"), String::from("E"));
        
        let result = read_serial_settings(&settings);

        assert_eq!(result.is_ok(), true);
        let value = result.unwrap();
        assert_eq!(value.parity_bit, ParityBitSetting::Even);
    }

    #[test]
    fn host_settings_single_pair() {
        let mut settings = HashMap::new();
        settings.insert(String::from("api_hosts"), String::from("localhost"));
        settings.insert(String::from("api_keys"), String::from("this-is-not-secret"));

        let result = read_host_settings(&settings);

        assert_eq!(result.is_ok(), true);
        let value = result.unwrap();
        assert_eq!(value.hosts.len(), 1);
        assert_eq!(value.hosts[0].address, "localhost");
        assert_eq!(value.hosts[0].key, "this-is-not-secret");
    }

    #[test]
    fn host_settings_no_api_hosts() {
        let mut settings = HashMap::new();
        settings.insert(String::from("api_hosts"), String::from("localhost"));

        let result = read_host_settings(&settings);

        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn host_settings_no_api_keys() {
        let mut settings = HashMap::new();
        settings.insert(String::from("api_hosts"), String::from("localhost"));

        let result = read_host_settings(&settings);

        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn host_settings_multiple_pairs() {
        let mut settings = HashMap::new();
        settings.insert(
            String::from("api_hosts"),
            String::from("localhost,remote-host"),
        );
        settings.insert(
            String::from("api_keys"),
            String::from("this-is-not-secret,this-better-be-secret"),
        );

        let result = read_host_settings(&settings);

        assert_eq!(result.is_ok(), true);
        let value = result.unwrap();
        assert_eq!(value.hosts.len(), 2);
        assert_eq!(value.hosts[0].address, "localhost");
        assert_eq!(value.hosts[0].key, "this-is-not-secret");
        assert_eq!(value.hosts[1].address, "remote-host");
        assert_eq!(value.hosts[1].key, "this-better-be-secret");
    }

    #[test]
    fn host_settings_number_elements_mismatch() {
        let mut settings = HashMap::new();
        settings.insert(
            String::from("api_hosts"),
            String::from("localhost,remote-host"),
        );
        settings.insert(String::from("api_keys"), String::from("this-is-not-secret"));

        let result = read_host_settings(&settings);

        assert_eq!(result.is_ok(), false);
    }
}
