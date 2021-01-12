pub mod reader;
pub mod sender;
pub mod settings;

pub trait TelegramConsumer {
    fn consume(&mut self, telegram: &str);
}
