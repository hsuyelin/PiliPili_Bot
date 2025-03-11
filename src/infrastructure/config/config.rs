use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::RwLock;
use once_cell::sync::Lazy;

use crate::{error_log, info_log};
use super::emby::EmbyConfig;

const CONFIG_LOGGER_DOMAIN: &str = "[CONFIG]";
const CONFIG_DIR: &str = "config";
const CONFIG_FILE: &str = "config/config.toml";
const TEMPLATE_FILE: &str = "src/infrastructure/config/config.template";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub emby: EmbyConfig,
}

pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    match Config::init() {
        Ok(config) => RwLock::new(config),
        Err(error) => {
            let message = format!("Failed to initialize configuration: {}", error);
            error_log!(CONFIG_LOGGER_DOMAIN, message);
            std::process::exit(1);
        }
    }
});

impl Config {

    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new(CONFIG_FILE);
        let config_dir = Path::new(CONFIG_DIR);

        if !config_dir.exists() {
            fs::create_dir(config_dir)?;
            let message = format!("📂 Create config directory: {}", CONFIG_DIR);
            info_log!(CONFIG_LOGGER_DOMAIN, message);
        }

        if !config_path.exists() {
            let template_path = Path::new(TEMPLATE_FILE);
            if template_path.exists() {
                fs::copy(template_path, config_path)?;
                let message = format!(
                    "📄 Copy default config: {} -> {} success!", 
                    template_path.display(), 
                    config_path.display()
                );
                info_log!(CONFIG_LOGGER_DOMAIN, message);
            } else {
                let message = format!("❌ Config template file missing: {}", TEMPLATE_FILE);
                error_log!(CONFIG_LOGGER_DOMAIN, message);
                return Err(
                    io::Error::new(
                        io::ErrorKind::NotFound, "Config template file missing"
                    )
                        .into()
                );
            }
        }

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_content)?;
        let message = format!("✅ Config load success at {}", config_path.display());
        info_log!(CONFIG_LOGGER_DOMAIN, message);

        Ok(config)
    }

    pub fn get() -> std::sync::RwLockReadGuard<'static, Config> {
        CONFIG.read().unwrap()
    }
}
