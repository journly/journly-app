use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct DbConfig {
    pub pg_host: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_port: String,
    pub pg_db: String,
}

#[derive(Clone, Debug)]
pub struct RedisConfig {
    pub redis_host: String,
    pub redis_port: String,
}

#[derive(Clone, Debug)]
pub struct Server {
    pub server_addr: String,
    pub server_port: String,
    pub db: DbConfig,
    pub redis: RedisConfig,
}

impl Server {
    pub fn build() -> Self {
        dotenv().ok();

        let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR must be set");
        let server_port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let pg_host = env::var("PG_HOST").expect("PG_HOST must be set");
        let pg_user = env::var("PG_USER").expect("PG_USER must be set");
        let pg_password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
        let pg_port = env::var("PG_PORT").expect("PG_PORT must be set");
        let pg_db = env::var("PG_DB").expect("PG_DB must be set");
        let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");
        let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT must be set");

        Self {
            server_addr,
            server_port,
            db: DbConfig {
                pg_host,
                pg_user,
                pg_password,
                pg_port,
                pg_db,
            },
            redis: RedisConfig {
                redis_host,
                redis_port,
            },
        }
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
