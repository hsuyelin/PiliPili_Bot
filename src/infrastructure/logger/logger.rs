use std::sync::Mutex;
use once_cell::sync::Lazy;

use super::level::LogLevel;
use super::internal_logger::InternalLogger;

pub static LOGGER: Lazy<Mutex<InternalLogger>> = Lazy::new(|| {
    Mutex::new(InternalLogger::new(
        LogLevel::Info,
        true,
        Some("logs".to_string()),
        10 * 1024 * 1024,
        3,
        Some("Logger".to_string()),
    ))
});

pub struct Logger;

impl Logger {

    pub fn log_level() -> LogLevel {
        let logger = LOGGER.lock().unwrap();
        logger.log_level
    }

    pub fn set_log_level(level: LogLevel) {
        let mut logger = LOGGER.lock().unwrap();
        logger.log_level = level;
    }

    pub fn set_log_enabled(enabled: bool) {
        let mut logger = LOGGER.lock().unwrap();
        logger.set_enabled(enabled);
    }
    
    pub fn debug(domain: String, message: &str) {
        Self::log(LogLevel::Debug, domain, message);
    }

    pub fn info(domain: String, message: &str) {
        Self::log(LogLevel::Info, domain, message);
    }

    pub fn warn(domain: String, message: &str) {
        Self::log(LogLevel::Warn, domain, message);
    }

    pub fn error(domain: String, message: &str) {
        Self::log(LogLevel::Error, domain, message);
    }
    
    pub fn rotate_logs() {
        let mut logger = LOGGER.lock().unwrap();
        logger.rotate_logs();
    }

    fn log(level: LogLevel, domain: String, message: &str) {
        let mut logger = LOGGER.lock().unwrap();
        let domain = if domain.is_empty() {
            "LOGGER".to_string()
        } else {
            domain.clone()
        };
        logger.set_domain(domain);
        logger.log(level, message);
    }
}