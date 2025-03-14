//! Provides convenient macros for logging at different levels.
//! 
//! This module exports macros that make it easy to log messages with different severity levels.
//! Each macro supports both a simple form (with just a message) and a form that includes a domain.

/// Log a message at the trace level.
/// 
/// This macro supports two forms:
/// 
/// 1. Simple form with just a message:
/// ```rust
/// trace_log!("This is a trace message");
/// ```
/// 
/// 2. Form with domain and message:
/// ```rust
/// trace_log!("[MyDomain]", "This is a trace message");
/// ```
/// 
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! trace_log {
    ($msg:expr) => {
        trace_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::trace!("{} {}", $domain, $msg);
    };
}

/// Log a message at the debug level.
/// 
/// This macro supports two forms:
/// 
/// 1. Simple form with just a message:
/// ```rust
/// debug_log!("This is a debug message");
/// ```
/// 
/// 2. Form with domain and message:
/// ```rust
/// debug_log!("[MyDomain]", "This is a debug message");
/// ```
/// 
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! debug_log {
    ($msg:expr) => {
        debug_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::debug!("{} {}", $domain, $msg);
    };
}

/// Log a message at the info level.
/// 
/// This macro supports two forms:
/// 
/// 1. Simple form with just a message:
/// ```rust
/// info_log!("This is an info message");
/// ```
/// 
/// 2. Form with domain and message:
/// ```rust
/// info_log!("[MyDomain]", "This is an info message");
/// ```
/// 
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! info_log {
    ($msg:expr) => {
        info_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::info!("{} {}", $domain, $msg);
    };
}

/// Log a message at the warn level.
/// 
/// This macro supports two forms:
/// 
/// 1. Simple form with just a message:
/// ```rust
/// warn_log!("This is a warning message");
/// ```
/// 
/// 2. Form with domain and message:
/// ```rust
/// warn_log!("[MyDomain]", "This is a warning message");
/// ```
/// 
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! warn_log {
    ($msg:expr) => {
        warn_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::warn!("{} {}", $domain, $msg);
    };
}

/// Log a message at the error level.
/// 
/// This macro supports two forms:
/// 
/// 1. Simple form with just a message:
/// ```rust
/// error_log!("This is an error message");
/// ```
/// 
/// 2. Form with domain and message:
/// ```rust
/// error_log!("[MyDomain]", "This is an error message");
/// ```
/// 
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! error_log {
    ($msg:expr) => {
        error_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::error!("{} {}", $domain, $msg);
    };
}