use once_cell::sync::Lazy;
use tracing_appender::non_blocking::WorkerGuard;
use std::sync::Arc;

use pilipili_bot::infrastructure::logger::builder::LoggerBuilder;
use pilipili_bot::infrastructure::logger::{LogLevel};

static _LOGGER: Lazy<Arc<WorkerGuard>> = Lazy::new(|| {
    Arc::new(LoggerBuilder::default()
        .with_level(LogLevel::Info)
        .init())
});

fn main() {
    let _guard = &_LOGGER;
}
