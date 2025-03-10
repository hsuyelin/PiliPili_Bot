use std::fmt::Debug;
use clia_local_offset::current_local_offset;
use time::macros::format_description;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::{self, time::OffsetTime};

use super::{LogLevel, LogWriter, LogRotation, LogFormat, LogDisplayOptions};

#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    level: LogLevel,
    writer: LogWriter,
    directory: String,
    file_prefix: String,
    rolling: LogRotation,
    format: LogFormat,
    display_options: LogDisplayOptions,
}

impl Default for LoggerBuilder {

    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            writer: LogWriter::File,
            directory: "./logs".to_owned(),
            file_prefix: "".to_owned(),
            rolling: LogRotation::Daily,
            format: LogFormat::Compact,
            display_options: LogDisplayOptions::default(),
        }
    }
}

impl LoggerBuilder {

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_writer(mut self, writer: LogWriter) -> Self {
        self.writer = writer;
        self
    }

    pub fn with_directory(mut self, directory: &str) -> Self {
        self.directory = directory.to_owned();
        self
    }

    pub fn with_file_prefix(mut self, file_prefix: &str) -> Self {
        self.file_prefix = file_prefix.to_owned();
        self
    }

    pub fn with_rolling(mut self, rolling: LogRotation) -> Self {
        self.rolling = rolling;
        self
    }

    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_display_options(mut self, display_options: LogDisplayOptions) -> Self {
        self.display_options = display_options;
        self
    }

    //noinspection DuplicatedCode
    pub fn init(self) -> WorkerGuard {
        let file_appender = self.rolling.create_file_appender(
            self.directory,
            self.file_prefix
        );
        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

        let offset = current_local_offset().expect("Can not get local offset!");
        let timer = OffsetTime::new(
            offset,
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        );

        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
                    tracing_subscriber::EnvFilter::new(self.level.to_string()),
                ),
            )
            .with_ansi(self.writer == LogWriter::Console);

        match self.format {
            LogFormat::Pretty => {
                let subscriber = subscriber
                    .event_format(
                        fmt::format()
                            .pretty()
                            .with_level(self.display_options.level)
                            .with_target(self.display_options.target)
                            .with_thread_ids(self.display_options.thread_ids)
                            .with_thread_names(self.display_options.thread_names)
                            .with_source_location(self.display_options.source_location)
                    )
                    .with_timer(timer);

                match self.writer {
                    LogWriter::File => {
                        subscriber.with_writer(file_writer).init();
                    }
                    LogWriter::Console => {
                        subscriber.with_writer(std::io::stdout).init();
                    }
                }
            },
            LogFormat::Compact => {
                let subscriber = subscriber
                    .event_format(
                        fmt::format()
                            .compact()
                            .with_level(self.display_options.level)
                            .with_target(self.display_options.target)
                            .with_thread_ids(self.display_options.thread_ids)
                            .with_thread_names(self.display_options.thread_names)
                            .with_source_location(self.display_options.source_location)
                    )
                    .with_timer(timer);

                match self.writer {
                    LogWriter::File => {
                        subscriber.with_writer(file_writer).init();
                    }
                    LogWriter::Console => {
                        subscriber.with_writer(std::io::stdout).init();
                    }
                }
            },
            LogFormat::Json => {
                let subscriber = subscriber
                    .event_format(
                        fmt::format()
                            .json()
                            .with_level(self.display_options.level)
                            .with_target(self.display_options.target)
                            .with_thread_ids(self.display_options.thread_ids)
                            .with_thread_names(self.display_options.thread_names)
                            .with_source_location(self.display_options.source_location)
                    )
                    .with_timer(timer);

                match self.writer {
                    LogWriter::File => {
                        subscriber.with_writer(file_writer).init();
                    }
                    LogWriter::Console => {
                        subscriber.with_writer(std::io::stdout).init();
                    }
                }
            },
            LogFormat::Full => {
                let subscriber = subscriber
                    .event_format(
                        fmt::format()
                            .with_level(self.display_options.level)
                            .with_target(self.display_options.target)
                            .with_thread_ids(self.display_options.thread_ids)
                            .with_thread_names(self.display_options.thread_names)
                            .with_source_location(self.display_options.source_location)
                    )
                    .with_timer(timer);

                match self.writer {
                    LogWriter::File => {
                        subscriber.with_writer(file_writer).init();
                    }
                    LogWriter::Console => {
                        subscriber.with_writer(std::io::stdout).init();
                    }
                }
            }
        }
        
        guard
    }
}