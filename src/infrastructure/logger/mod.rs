//! A flexible and configurable logging system for Rust applications.
//! 
//! This module provides a comprehensive logging solution with the following features:
//! - Configurable log levels
//! - Log rotation support
//! - Builder pattern for easy configuration
//! - Convenient macros for logging
//! 
//! # Examples
//! 
//! ```rust
//! use infrastructure::logger::{LogLevel, LogRotation, LoggerBuilder};
//! 
//! // Configure and initialize the logger
//! LoggerBuilder::default()
//!     .with_level(LogLevel::Info)
//!     .with_rolling(LogRotation::Daily)
//!     .with_directory("logs")
//!     .with_file_prefix("app")
//!     .init();
//! 
//! // Use the logging macros
//! info_log!("Application started");
//! debug_log!("[Database]", "Connected to database");
//! ```

pub mod builder;
pub mod rotation;
pub mod level;
pub mod macros;

pub use builder::LoggerBuilder;
pub use rotation::LogRotation;
pub use level::LogLevel;