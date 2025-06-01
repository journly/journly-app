use config::Config;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DbConfig {
    pub pg_host: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_port: String,
    pub pg_db: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfig {
    pub redis_host: String,
    pub redis_port: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JournlyConfig {
    pub server_addr: String,
    pub server_port: String,
    pub db_config: DbConfig,
    pub redis_config: RedisConfig,
}

impl JournlyConfig {
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

impl DbConfig {
    pub fn get_db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.pg_user, self.pg_password, self.pg_host, self.pg_port, self.pg_db
        )
    }
}

impl RedisConfig {
    pub fn get_redis_url(&self) -> String {
        format!("redis://{}:{}", self.redis_host, self.redis_port)
    }
}
