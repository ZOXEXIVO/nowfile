extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::*;

pub struct Logger;

impl Logger{
    pub fn init(system: &'static str) -> slog::Logger {
        let decorator = slog_term::PlainDecorator::new(std::io::stdout());

        let drain = slog_async::Async::new(
            slog_term::CompactFormat::new(decorator).build().fuse()
        ).build().fuse();

        return slog::Logger::root(drain, o!("system" => system));
    }
}