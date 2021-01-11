use super::settings;

use serialport;
use std::borrow::Cow;
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::time::Duration;

pub trait TelegramConsumer {
    fn consume(&mut self, telegram: &str) -> ();
}
struct PrintConsumer {}
impl PrintConsumer {
    fn new() -> Self {
        PrintConsumer {}
    }
}
impl TelegramConsumer for PrintConsumer {
    fn consume(&mut self, telegram: &str) -> () {
        println!("Found telegram:\n{}", telegram)
    }
}

fn find_start_of_telegram(buffer: &str) -> Option<usize> {
    buffer.find("/")
}

fn find_end_of_telegram(buffer: &str, from: usize) -> Option<usize> {
    match buffer[from..].find('!') {
        Some(excl_mark) => {
            // Not all meters send the checksum; some simply have a last line with just '!'
            match buffer[excl_mark..].find('\n') {
                Some(line_end) => {
                    let end = excl_mark + line_end;
                    log::trace!(
                        "Exclamation mark found at index {}, returning index {}",
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
fn eat_telegrams<'a>(buffer: &'a str, consumer: &mut dyn TelegramConsumer) -> Cow<'a, str> {
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

fn read_from_serial_port(port: &mut dyn serialport::SerialPort) {
    let reader = &mut BufReader::new(port);

    let mut consumer = PrintConsumer::new();

    let mut buffer = String::new();
    loop {
        reader.read_line(&mut buffer).expect("Could not read data");

        let clone = buffer.clone();

        let new_buffer = eat_telegrams(&clone, &mut consumer);

        if buffer != new_buffer {
            log::trace!("Replacing buffer {} with {}", buffer, new_buffer);
            buffer.clear();
            buffer.push_str(&new_buffer);
        }
    }
}

pub fn connect_to_meter(serial_settings: settings::SerialSettings) {
    log::info!(
        "Connecting to {} using baud rate {}",
        &serial_settings.port,
        &serial_settings.baud_rate
    );

    let mut port = serialport::new(&serial_settings.port, serial_settings.baud_rate)
        .data_bits(serialport::DataBits::Seven)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::Even)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_secs(20))
        .open()
        .expect("Failed to open port");

    read_from_serial_port(&mut *port);
}

#[cfg(test)]
mod tests {
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
    impl TelegramConsumer for TestConsumer {
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
}
