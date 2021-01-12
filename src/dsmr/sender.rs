use super::settings;

use std::collections::HashMap;

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
    fn consume(&mut self, telegram: &str) {
        log::trace!("- uploading telegram to {}", self.host);
        let url = [&self.host, "/api/v1/datalogger/dsmrreading"].join("");

        let mut params = HashMap::new();
        params.insert("telegram", telegram.to_string());

        let result = self
            .client
            .post(&url)
            .header("Authorization", format!("Token {}", self.key))
            .form(&params)
            .send();

        match result {
            Ok(response) => log::trace!("Got response with status {}", response.status()),
            Err(msg) => log::warn!("Could not upload telegram due to {}", msg),
        }
    }
}

pub struct DelegatingConsumer {
    delegates: Vec<UploadConsumer>,
}
impl DelegatingConsumer {
    pub fn new(targets: Vec<settings::Host>) -> Self {
        let delegates = (0..targets.len())
            .map(|index| UploadConsumer::new(&targets[index]))
            .collect::<Vec<UploadConsumer>>();

        DelegatingConsumer { delegates }
    }
}
impl super::TelegramConsumer for DelegatingConsumer {
    fn consume(&mut self, telegram: &str) {
        log::info!("Uploading telegram to {} hosts", self.delegates.len());
        for delegate in &mut self.delegates {
            delegate.consume(telegram);
        }
    }
}
