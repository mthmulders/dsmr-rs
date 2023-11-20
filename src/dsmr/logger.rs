pub struct LoggingConsumer {
    host_counter: u32,
    telegram_counter: u32,
}
impl LoggingConsumer {
    pub fn new(host_counter: u32) -> Self {
        LoggingConsumer {
            host_counter,
            telegram_counter: 0,
        }
    }
}
impl super::TelegramConsumer for LoggingConsumer {
    fn consume(&mut self, _telegram: &str) -> bool {
        self.telegram_counter += 1;
        if self.telegram_counter == 10000 {
            log::info!("Uploaded 10000 telegrams to {} host(s)", self.host_counter);
            self.telegram_counter = 0;
        }

        true
    }
}
