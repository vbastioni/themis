use std::fmt::Display;

use secrecy::{Secret, ExposeSecret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

use crate::error::AppError::{self, Environment as EnvError};

#[derive(Deserialize, Debug)]
pub struct Setting {
    pub application: ApplicationSetting,
    pub postgres: PostgresSetting,
    pub meilisearch: MeiliSetting,
}

impl Setting {
    /// An alias of configuration::get_configuration, so you can write
    /// configuration::Setting::get()
    pub fn get() -> Result<Setting, config::ConfigError> {
        get_configuration()
    }
}

#[derive(Deserialize, Debug)]
pub struct ApplicationSetting {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct PostgresSetting {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: Secret<String>,
    pub database_name: String,
    pub require_ssl: bool,
}

impl PostgresSetting {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    pub fn with_db(&self) -> PgConnectOptions {
        #[allow(unused_mut)]
        let mut options = self.without_db().database(&self.database_name);
        // options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}

#[derive(Deserialize, Debug)]
pub struct MeiliSetting {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub key: Secret<String>,
}

pub fn get_configuration() -> Result<Setting, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")).required(false))
        .add_source(
            config::File::from(configuration_directory.join(environment.as_str())).required(false),
        )
        .add_source(config::Environment::default().prefix("app").separator("__"))
        .build()?
        .try_deserialize()
}

pub enum Environment {
    Local,
    Dev,
    Prod,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Dev => "dev",
            Environment::Prod => "production",
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<String> for Environment {
    type Error = AppError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "dev" | "development" => Ok(Environment::Dev),
            "production" | "prod" => Ok(Environment::Prod),
            _ => Err(EnvError { env: s }),
        }
    }
}
