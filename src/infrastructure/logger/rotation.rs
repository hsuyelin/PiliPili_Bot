use tracing_appender::rolling::{self, RollingFileAppender};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogRotation {
    Minutely,
    Hourly,
    Daily,
    Never,
}

impl LogRotation {

    pub fn create_file_appender(
        self, 
        directory: String, 
        file_prefix: String
    ) -> RollingFileAppender {
        match self {
            LogRotation::Minutely => rolling::minutely(directory, file_prefix),
            LogRotation::Hourly => rolling::hourly(directory, file_prefix),
            LogRotation::Daily => rolling::daily(directory, file_prefix),
            LogRotation::Never => rolling::never(directory, file_prefix),
        }
    }
}