use confik::{Configuration, FileSource};
use serde::Deserialize;

#[derive(Debug, Default, Configuration)]
pub struct JournalyConfig {
    pub server_addr: String,
    pub dev_port: u16,
    #[confik(from = DbConfig)]
    pub db_config: deadpool_postgres::Config,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct DbConfig(deadpool_postgres::Config);

impl From<DbConfig> for deadpool_postgres::Config {
    fn from(value: DbConfig) -> Self {
        value.0
    }
}

impl confik::Configuration for DbConfig {
    type Builder = Option<Self>;
}

pub fn get_configuration() -> Result<JournalyConfig, confik::Error> {
    JournalyConfig::builder()
        .override_with(FileSource::new("config.toml"))
        .try_build()
}
