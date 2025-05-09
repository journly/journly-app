use confik::{Configuration, FileSource};
use journaly_server::config::JournalyConfig;

#[test]
pub fn build_config() {
    let config = JournalyConfig::builder()
        .override_with(FileSource::new("dev.toml"))
        .try_build()
        .expect("Failed to build config");

    assert_eq!(config.server_addr, "127.0.0.1".to_string());
    assert_eq!(config.dev_port, 8080);
    assert_eq!(config.db_config.host.unwrap(), "localhost".to_string());
    assert_eq!(config.db_config.user.unwrap(), "postgres".to_string());
    assert_eq!(config.db_config.password.unwrap(), "postgres".to_string());
    assert_eq!(config.db_config.dbname.unwrap(), "test".to_string());
    assert_eq!(config.db_config.port.unwrap(), 5431);
    assert_eq!(config.db_config.pool.unwrap().max_size, 16);
}
