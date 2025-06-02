use config::Config;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PgConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub port: String,
    pub db: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub server_addr: String,
    pub server_port: String,
    pub postgres: PgConfig,
    pub redis: RedisConfig,
    pub mailgun_smtp: EmailConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EmailConfig {
    pub smtp_login: String,
    pub smtp_password: String,
    pub smtp_server: String,
}

impl Server {
    pub fn build(config_path: &str) -> Self {
        let configuration = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()
            .unwrap();

        configuration
            .try_deserialize()
            .expect("Failed to build from config file")
    }
}

impl PgConfig {
    pub fn get_db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }
}

impl RedisConfig {
    pub fn get_redis_url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}
