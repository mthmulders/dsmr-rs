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
    fn consume(&mut self, _telegram: &str) {
        self.telegram_counter += 1;
        // We expect a telegram every 10 seconds -> 6 per minute -> 360 per hour.
        if self.telegram_counter == 360 {
            log::info!("Uploaded 360 telegrams to {} host(s)", self.host_counter);
            self.telegram_counter = 0;
        }
    }
}
