use super::settings;

struct UploadConsumer {
    host: String,
    key: String,
}
impl UploadConsumer {
    fn new(target: &settings::Host) -> Self {
        UploadConsumer { host: String::from(&target.address), key: String::from(&target.key) }
    }
}
impl super::TelegramConsumer for UploadConsumer {
    fn consume(&mut self, telegram: &str) {
        println!("About to upload telegram to {}", self.host);
    }
}

struct DelegatingConsumer {
    delegates: Vec<UploadConsumer>,
}
impl DelegatingConsumer {
    fn new(targets: Vec<settings::Host>) -> Self {

        let delegates = (0..targets.len())
            .map(|index| UploadConsumer::new(&targets[index]))
            .collect::<Vec<UploadConsumer>>();

        DelegatingConsumer { delegates: delegates }
    }
}
impl super::TelegramConsumer for DelegatingConsumer {
    fn consume(&mut self, telegram: &str) {
        println!("About to upload telegram to {} hosts", self.delegates.len());
        for delegate in &mut self.delegates {
            delegate.consume(telegram);
        }
    }
}