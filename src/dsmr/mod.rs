pub mod reader;
pub mod sender;
pub mod settings;
pub mod logger;

pub trait TelegramConsumer {
    fn consume(&mut self, telegram: &str);
}
