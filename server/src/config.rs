use config::Config;
use serde::Deserialize;

use crate::auth::JwtConfig;

#[derive(Clone, Debug, Deserialize)]
pub struct PgConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub port: String,
    pub db: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BaseConfig {
    pub production: bool,
    pub domain_name: String,
    pub ip_address: String,
    pub port: String,
    pub allowed_origins: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub base: BaseConfig,
    pub postgres: PgConfig,
    pub mailgun_smtp: SmtpConfig,
    pub google_oauth: GoogleOAuthConfig,
    pub jwt_config: JwtConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SmtpConfig {
    pub smtp_login: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_server: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
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
