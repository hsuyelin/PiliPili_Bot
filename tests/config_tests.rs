/*
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::sync::RwLockReadGuard;
    use indoc::indoc;
    use pilipili_bot::infrastructure::config::Config;

    const CONFIG_DIR: &str = "config";
    const CONFIG_FILE: &str = "config/config.toml";
    const TEMPLATE_FILE: &str = "src/infrastructure/config/config.template";

    fn setup() {
        let _ = fs::create_dir_all(CONFIG_DIR);

        let test_config = indoc! {r#"
    [emby]
    base_url = "https://test.emby.server"
    api_key = "test_api_key"
"#};
        fs::write(CONFIG_FILE, test_config).expect("Failed to write test config");

        let template_config = indoc! {r#"
    [emby]
    base_url = "http://127.0.0.1:8096"
    api_key = "your_emby_api_key"
"#};
        fs::write(TEMPLATE_FILE, template_config).expect("Failed to write template config");
    }

    #[test]
    fn test_load_existing_config() {
        setup();
        let config: RwLockReadGuard<'static, Config> = Config::get();

        assert_eq!(config.emby.base_url, "https://test.emby.server");
        assert_eq!(config.emby.api_key, "test_api_key");
    }

    #[test]
    fn test_create_config_from_template() {
        setup();
        if Path::new(CONFIG_FILE).exists() {
            fs::remove_file(CONFIG_FILE).expect("Failed to remove config file");
        }

        let config: RwLockReadGuard<'static, Config> = Config::get();

        assert!(Path::new(CONFIG_FILE).exists(), "Config file should be created");
        assert_eq!(config.emby.base_url, "http://127.0.0.1:8096");
        assert_eq!(config.emby.api_key, "your_emby_api_key");
    } 
}
*/