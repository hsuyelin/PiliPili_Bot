#![allow(dead_code)]
#![allow(unused_variables)]

use crate::logger::level::LogLevel;
use super::internal_logger::InternalLogger;
use std::sync::Mutex;
use once_cell::sync::Lazy;

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

    pub fn debug(domain: Option<String>, message: &str) {
        Self::log(LogLevel::Debug, domain, message);
    }

    pub fn info(domain: Option<String>, message: &str) {
        Self::log(LogLevel::Info, domain, message);
    }

    pub fn warn(domain: Option<String>, message: &str) {
        Self::log(LogLevel::Warn, domain, message);
    }

    pub fn error(domain: Option<String>, message: &str) {
        Self::log(LogLevel::Error, domain, message);
    }
    
    pub fn rotate_logs() -> bool {
        let mut logger = LOGGER.lock().unwrap();
        logger.rotate_logs()
    }
    
    pub fn set_log_enabled(enabled: bool) {
        let mut logger = LOGGER.lock().unwrap();
        logger.set_enabled(enabled);
    }

    fn log(level: LogLevel, domain: Option<String>, message: &str) {
        let mut logger = LOGGER.lock().unwrap();
        let domain = domain.unwrap_or_else(|| "Logger".to_string());
        logger.set_domain(domain);
        logger.set_level(level);
        logger.log(level, message);
    }
}