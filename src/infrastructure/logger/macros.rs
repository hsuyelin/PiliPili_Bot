#[macro_export]
macro_rules! trace_log {
    ($msg:expr) => {
        trace_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::trace!("{} {}", $domain, $msg);
    };
}

#[macro_export]
macro_rules! debug_log {
    ($msg:expr) => {
        debug_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::debug!("{} {}", $domain, $msg);
    };
}

#[macro_export]
macro_rules! info_log {
    ($msg:expr) => {
        info_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::info!("{} {}", $domain, $msg);
    };
}

#[macro_export]
macro_rules! warn_log {
    ($msg:expr) => {
        warn_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::warn!("{} {}", $domain, $msg);
    };
}

#[macro_export]
macro_rules! error_log {
    ($msg:expr) => {
        error_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::error!("{} {}", $domain, $msg);
    };
}