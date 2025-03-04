use std::env;
use chrono::Local;
use std::fs;
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use colored::Colorize;

use crate::infrastructure::logger::level::LogLevel;

pub struct InternalLogger {

    pub log_level: LogLevel,
    pub is_enabled: bool,
    log_dir: Option<String>,
    max_log_size: u64,
    max_log_count: usize,
    log_file: Option<BufWriter<fs::File>>,
    domain: Option<String>,
    current_log_path: Option<PathBuf>,
}

impl InternalLogger {

    pub fn new(
        log_level: LogLevel,
        is_enabled: bool,
        log_dir: Option<String>,
        max_log_size: u64,
        max_log_count: usize,
        domain: Option<String>,
    ) -> Self {
        let mut logger = InternalLogger {
            log_level,
            is_enabled,
            log_dir,
            max_log_size,
            max_log_count,
            log_file: None,
            domain: Some(domain.unwrap_or_else(|| "Logger".to_string())),
            current_log_path: None,
        };
        logger.init_log_file();
        logger
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = Some(domain);
    }

    pub fn set_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        if !self.is_enabled || level < self.log_level {
            return;
        }

        self.log_to_console(level, message);
        if self.log_dir.as_ref().map(|s| !s.is_empty()).unwrap_or(false) {
            self.init_log_file();
            self.log_to_file(level, message);
        }
    }

    pub fn rotate_logs(&mut self) -> bool {
        if let Some(path) = self.log_dir.take() {
            let log_file_path = Path::new(&path);
            if let Ok(metadata) = fs::metadata(log_file_path) {
                if metadata.len() > self.max_log_size {
                    return self.perform_log_rotation(log_file_path);
                }
            } else {
                eprintln!("Failed to get log file metadata.");
            }
            self.log_dir = Some(path);
        }
        false
    }

    fn init_log_file(&mut self) {
        if let Some(log_dir) = &self.log_dir {
            let absolute_log_dir = self.get_absolute_log_dir(log_dir);

            if !absolute_log_dir.exists() {
                if let Err(e) = fs::create_dir_all(&absolute_log_dir) {
                    eprintln!("Failed to create log directory: {}", e);
                    return;
                } else {
                    eprintln!("Log directory created at: {}", absolute_log_dir.display());
                }
            }

            let date_str = Local::now().format("%Y-%m-%d").to_string();
            let log_file_path = absolute_log_dir.join(format!("{}.log", date_str));

            if self.current_log_path.as_ref() == Some(&log_file_path) {
                return;
            }

            match fs::OpenOptions::new().create(true).append(true).open(&log_file_path) {
                Ok(file) => {
                    self.log_file = Some(BufWriter::new(file));
                    self.current_log_path = Some(log_file_path.clone());
                    eprintln!("Log file initialized at: {}", log_file_path.display());
                }
                Err(e) => {
                    eprintln!("Failed to open log file: {}", e);
                }
            }
        } else {
            eprintln!("Log directory is not specified.");
        }
    }

    fn log_to_file(&mut self, level: LogLevel, message: &str) {
        if let Some(file_writer) = self.log_file.as_mut() {
            let level_str = match level {
                LogLevel::Debug => "DEBUG",
                LogLevel::Info => "INFO",
                LogLevel::Warn => "WARN",
                LogLevel::Error => "ERROR",
            };

            let now = Local::now().format("%Y-%m-%d %H:%M:%S");
            if let Err(e) = writeln!(
                file_writer,
                "{} [{}] {} - {}",
                now.to_string(),
                level_str,
                self.domain.as_deref().unwrap_or("Logger"),
                message
            ) {
                eprintln!("Failed to write to log file: {}", e);
            }

            if let Err(e) = file_writer.flush() {
                eprintln!("Failed to flush log file: {}", e);
            }
        }
    }
    
    fn log_to_console(&self, level: LogLevel, message: &str) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level_str = match level {
            LogLevel::Debug => "DEBUG".blue(),
            LogLevel::Info => "INFO".green(),
            LogLevel::Warn => "WARN".yellow(),
            LogLevel::Error => "ERROR".red(),
        };
        println!(
            "{} [{}] {} - {}",
            now.to_string().bold(),
            level_str,
            self.domain.as_ref().unwrap_or(&"Logger".to_string()),
            message
        );
    }

    fn perform_log_rotation(&mut self, log_file_path: &Path) -> bool {
        let date_str = Local::now().format("%Y-%m-%d").to_string();
        let new_log_file = log_file_path.with_file_name(format!("{}.log.old", date_str));

        if let Err(e) = fs::rename(log_file_path, &new_log_file) {
            eprintln!("Log file rotation failed: {}", e);
            return false;
        } else {
            println!("Log file rotated to: {}", new_log_file.display());
        }

        self.cleanup_old_logs();
        self.init_log_file();
        true
    }

    fn cleanup_old_logs(&mut self) {
        if let Some(path) = &self.log_dir {
            let log_dir = Path::new(path);
            let mut log_files: Vec<_> = fs::read_dir(log_dir)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry.path().extension() == Some(std::ffi::OsStr::new("log")))
                .collect();

            log_files.sort_by(|a, b| {
                a.metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .cmp(&b.metadata().unwrap().modified().unwrap())
            });

            while log_files.len() > self.max_log_count {
                let file_to_delete = log_files.remove(0);
                let file_path = file_to_delete.path();
                if let Err(e) = fs::remove_file(&file_path) {
                    eprintln!("Failed to delete old log file: {}", e);
                } else {
                    println!("Deleted old log file: {}", file_path.display());
                }
            }
        }
    }

    fn get_absolute_log_dir(&self, log_dir: &str) -> PathBuf {
        if Path::new(log_dir).is_absolute() {
            Path::new(log_dir).to_path_buf()
        } else {
            env::current_dir().unwrap().join(log_dir)
        }
    }
}