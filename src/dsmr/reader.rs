use super::settings;
use super::settings::ParityBitSetting;

use serialport::SerialPort;

use std::borrow::Cow;
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::time::Duration;

fn find_start_of_telegram(buffer: &str) -> Option<usize> {
    buffer.find('/')
}

fn find_end_of_telegram(buffer: &str, from: usize) -> Option<usize> {
    match buffer[from..].find('!') {
        Some(excl_mark) => {
            let remainder = &buffer[(from + excl_mark + 1)..];
            // Not all meters send the checksum; some simply have a last line with just '!'
            match remainder.find('\n') {
                Some(line_end) => {
                    let end = from + excl_mark + line_end + 1;
                    log::trace!(
                        "Exclamation mark with line ending found at index {}, returning index {}",
                        excl_mark,
                        end
                    );
                    Some(end)
                }
                None => {
                    log::trace!(
                        "Exclamation mark found at index {}, but lines doesn't end",
                        excl_mark
                    );
                    None
                }
            }
        }
        None => None,
    }
}

// Scans the buffer to see if there is a telegram in it.
// If so, invokes the consumer with that telegram.
// Otherwise, clear the buffer (if there is no telegram unders construction)
// or keep it intact (if a telegram is under construction).
// Returns the new state of the buffer - which may be the unmodified buffer, or a new buffer.
fn eat_telegrams<'a>(buffer: &'a str, consumer: &mut dyn super::TelegramConsumer) -> Cow<'a, str> {
    let start_index = find_start_of_telegram(buffer);
    let end_index = find_end_of_telegram(buffer, start_index.unwrap_or(0));

    match (start_index, end_index) {
        (Some(start), Some(end)) => {
            let telegram = &buffer[start..(end + 1)];
            log::trace!(
                "Start and end found, consuming complete telegram from index {} to {}:\n{}",
                start,
                end,
                telegram
            );
            consumer.consume(telegram);
            let new_buffer = &buffer.replace(telegram, "");
            log::trace!("Buffer {} truncated to {}", buffer, new_buffer);
            Cow::Owned(String::from(new_buffer))
        }
        (None, Some(_)) => {
            log::trace!("No start of telegram, clearing buffer");
            Cow::Owned(String::new())
        }
        (Some(_), _) => {
            log::trace!("There is no end of the telegram, keeping buffer {}", buffer);
            Cow::Borrowed(buffer)
        }
        (None, None) => {
            log::trace!(
                "There is no start and no end of the telegram, keeping buffer {}",
                buffer
            );
            Cow::Borrowed(buffer)
        }
    }
}

pub fn read_from_serial_port(
    port: Box<dyn SerialPort>,
    consumer: &mut dyn super::TelegramConsumer,
) {
    let reader = &mut BufReader::new(port);

    // let mut consumer = PrintConsumer::new();

    let mut buffer = String::new();
    loop {
        let result = reader.read_line(&mut buffer);

        if result.is_err() {
            let error = result.expect_err("Expected an error, but there is none?");
            log::info!("Read error {}, clearing buffer", error.to_string());
            // Just drop this telegram
            buffer.clear();
        } else {
            let clone = buffer.clone();

            let new_buffer = eat_telegrams(&clone, consumer);

            if buffer != new_buffer {
                log::trace!("Replacing buffer {} with {}", buffer, new_buffer);
                buffer.clear();
                buffer.push_str(&new_buffer);
            }
        }
    }
}

pub fn connect_to_meter(serial_settings: settings::SerialSettings) -> Box<dyn SerialPort> {
    log::info!(
        "Connecting to {} using baud rate {}, byte size {} and parity bit {:#?}",
        &serial_settings.port,
        &serial_settings.baud_rate,
        &serial_settings.byte_size,
        &serial_settings.parity_bit
    );

    return serialport::new(&serial_settings.port, serial_settings.baud_rate)
        .data_bits(to_databits(&serial_settings.byte_size))
        .flow_control(serialport::FlowControl::None)
        .parity(to_serial_port_parity_bit(&serial_settings.parity_bit))
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_secs(20))
        .open()
        .expect("Failed to open port");
}

fn to_serial_port_parity_bit(input: &ParityBitSetting) -> serialport::Parity {
    match input {
        ParityBitSetting::Even => serialport::Parity::Even,
        ParityBitSetting::None => serialport::Parity::None,
        ParityBitSetting::Odd => serialport::Parity::Odd,
    }
}

fn to_databits(input: &u8) -> serialport::DataBits {
    match input {
        0..=4 => serialport::DataBits::Eight,
        5 => serialport::DataBits::Five,
        6 => serialport::DataBits::Six,
        7 => serialport::DataBits::Seven,
        8 => serialport::DataBits::Eight,
        9..=u8::MAX => serialport::DataBits::Eight,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn find_start_of_telegram() {
        assert_eq!(super::find_start_of_telegram("abcd\r\n/ISk5\\2"), Some(6));
        assert_eq!(super::find_start_of_telegram("abcd\r\nefgh"), None);
    }

    #[test]
    fn find_end_of_telegram() {
        assert_eq!(super::find_end_of_telegram("abcd\r\n!522B", 0), None);
        assert_eq!(
            super::find_end_of_telegram("abcd\r\n!522B\r\n", 0),
            Some(12)
        );
        assert_eq!(super::find_end_of_telegram("abcd\r\n!\r\n", 0), Some(8));
        assert_eq!(
            super::find_end_of_telegram("!522B\r\n\r\nabcd\r\n\r\n", 10),
            None
        );
    }

    struct TestConsumer {
        invoked: bool,
        telegram: String,
    }
    impl TestConsumer {
        fn new() -> Self {
            TestConsumer {
                invoked: false,
                telegram: String::new(),
            }
        }
    }
    impl super::super::TelegramConsumer for TestConsumer {
        fn consume(&mut self, telegram: &str) -> () {
            self.invoked = true;
            self.telegram = String::from(telegram);
        }
    }

    #[test]
    fn eat_telegrams_only_start() {
        let text = String::from("/ISk5\\2MT382-1000\r\n\r\n1-3:0.2.8(40)");
        let mut consumer = TestConsumer::new();

        let result = eat_telegrams(&text, &mut consumer);

        assert_eq!(result, text);
        assert_eq!(consumer.invoked, false);

        let text = String::from("/ISk5\\2MT382-1000\r\n\r\n1-3:0.2.8(40)!");
        let mut consumer = TestConsumer::new();

        let result = eat_telegrams(&text, &mut consumer);

        assert_eq!(result, text);
        assert_eq!(consumer.invoked, false);
    }

    #[test]
    fn eat_telegrams_only_end() {
        let text = String::from("0-1:24.4.0(1)\r\n!522B\r\n");
        let mut consumer = TestConsumer::new();

        let result = eat_telegrams(&text, &mut consumer);

        assert_eq!(result, "");
        assert_eq!(consumer.invoked, false);
    }

    #[test]
    fn eat_telegrams_start_and_end() {
        let mut text = String::from(
            "/ISk5\\2MT382-1000\r\n\r\n1-3:0.2.8(40)\r\n!522B\r\n\r\n/ISk5\\2MT382-1000",
        );
        let mut consumer = TestConsumer::new();

        let result = eat_telegrams(&mut text, &mut consumer);

        assert_eq!(consumer.invoked, true);
        assert_eq!(
            consumer.telegram,
            "/ISk5\\2MT382-1000\r\n\r\n1-3:0.2.8(40)\r\n!522B\r\n"
        );
        assert_eq!(result, "\r\n/ISk5\\2MT382-1000");
    }

    #[test]
    fn eat_telegrams_without_checksum_start_and_end() {
        let mut text =
            String::from("/ISk5\\2MT382-1000\r\n\r\n1-3:0.2.8(40)\r\n!\r\n\r\n/ISk5\\2MT382-1000");
        let mut consumer = TestConsumer::new();

        let result = eat_telegrams(&mut text, &mut consumer);

        assert_eq!(consumer.invoked, true);
        assert_eq!(
            consumer.telegram,
            "/ISk5\\2MT382-1000\r\n\r\n1-3:0.2.8(40)\r\n!\r\n"
        );
        assert_eq!(result, "\r\n/ISk5\\2MT382-1000");
    }

    #[test]
    fn eat_complete_telegram() {
        let mut input = read_test_resource("input1.txt".into());
        let mut consumer = TestConsumer::new();

        let _result = eat_telegrams(&mut input, &mut consumer);

        assert_eq!(consumer.invoked, true);
        assert_eq!(consumer.telegram, read_test_resource("output1.txt".into()),);
    }

    fn read_test_resource(path: PathBuf) -> String {
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("resources/test/");
        test_file.push(path);

        let mut binding = fs::read_to_string(test_file).expect("Failed to read file");
        let text = binding.as_mut_str();
        return String::from(text);

        // return fs::read_to_string(test_file).expect("Failed to read file").as_mut_str();
    }
}
