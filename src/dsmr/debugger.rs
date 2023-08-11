// For troubleshooting purposes...
// struct PrintConsumer {}
// impl PrintConsumer {
//     fn new() -> Self {
//         PrintConsumer {}
//     }
// }
// impl super::TelegramConsumer for PrintConsumer {
//     fn consume(&mut self, telegram: &str) {
//         println!("Found telegram:\n{}", telegram)
//     }
// }