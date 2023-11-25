use super::settings;

use std::collections::HashMap;

use crate::dsmr::logger::LoggingConsumer;
use crate::dsmr::TelegramConsumer;

struct UploadConsumer {
    host: String,
    key: String,
    client: reqwest::blocking::Client,
}
impl UploadConsumer {
    fn new(target: &settings::Host) -> Self {
        UploadConsumer {
            client: reqwest::blocking::Client::new(),
            host: String::from(&target.address),
            key: String::from(&target.key),
        }
    }
}
impl super::TelegramConsumer for UploadConsumer {
    fn consume(&mut self, telegram: &str) -> bool {
        log::trace!("- uploading telegram to {}", self.host);
        let url = [&self.host, "/api/v1/datalogger/dsmrreading"].join("");

        let mut params = HashMap::new();
        params.insert("telegram", telegram.to_string());

        let result = self
            .client
            .post(url)
            .header("Authorization", format!("Token {}", self.key))
            .form(&params)
            .send();

        match result {
            Ok(response) => {
                log::trace!("Got response with status {}", response.status());
                true
            }
            Err(msg) => {
                log::warn!("Could not upload telegram due to {}", msg);
                false
            }
        }
    }
}

pub struct DelegatingConsumer {
    delegates: Vec<Box<dyn TelegramConsumer>>,
    logger: LoggingConsumer,
}
impl DelegatingConsumer {
    pub fn new(targets: &Vec<settings::Host>) -> Self {
        let mut delegates: Vec<Box<dyn TelegramConsumer>> = Vec::with_capacity(targets.len() + 1);

        let logger: LoggingConsumer = LoggingConsumer::new(targets.len() as u32);

        (0..targets.len())
            .map(|index| UploadConsumer::new(&targets[index]))
            .map(Box::new)
            .for_each(|b| delegates.push(b));

        DelegatingConsumer { delegates, logger }
    }
}
impl super::TelegramConsumer for DelegatingConsumer {
    fn consume(&mut self, telegram: &str) -> bool {
        for delegate in &mut self.delegates {
            if delegate.consume(telegram) {
                self.logger.consume(telegram);
            }
        }

        true // ignored anyway...
    }
}
