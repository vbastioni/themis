use std::path;

use secrecy::Secret;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct Setting {
    pub meilisearch: MeiliSetting,
    pub elasticsearch: ElasticSetting,
}

impl Setting {
    /// An alias of configuration::get_configuration, so you can write
    /// configuration::Setting::get()
    pub fn get<P>(file: Option<P>) -> Result<Setting, config::ConfigError>
    where
        P: AsRef<path::Path>,
    {
        get_configuration(file)
    }
}

#[derive(Deserialize, Debug)]
pub struct ElasticSetting {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub user: String,
    pub pass: Secret<String>,
    pub cert: Secret<String>,
}

#[derive(Deserialize, Debug)]
pub struct MeiliSetting {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub key: Secret<String>,
}

impl MeiliSetting {
    pub fn url(&self) -> impl Into<String> {
        format!("http://{}:{}", self.host, self.port)
    }
}

pub fn get_configuration<P>(file: Option<P>) -> Result<Setting, config::ConfigError>
where
    P: AsRef<path::Path>,
{
    let mut builder = config::Config::builder();
    if let Some(p) = file {
        builder = builder.add_source(config::File::from(p.as_ref()).required(false));
    }
    builder
        .set_default("meilisearch.host", "localhost")?
        .set_default("meilisearch.port", "7700")?
        .set_default("elasticsearch.host", "localhost")?
        .set_default("elasticsearch.port", "9200")?
        .set_default("elasticsearch.user", "elastic")?
        .add_source(config::Environment::default().prefix("app").separator("__"))
        .build()?
        .try_deserialize()
}
