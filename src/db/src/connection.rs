use std::{env, path::PathBuf};

use cross_cutting::EnvironmentVariables;
use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};

#[derive(Clone)]
pub struct MongodbCliente {
    database_url: String,
    db_name: String,
}

pub struct MongoClint;


impl MongoClint {
    pub async fn conn() -> mongodb::Database {
        let conection_string = EnvironmentVariables::get_mongo_connection_string().unwrap();
        let db_name = EnvironmentVariables::get_mongo_db_name().unwrap();

        let client = mongodb::Client::with_uri_str(&conection_string).await;
        let db = client.unwrap().database(&db_name);

        db
    }

    pub async fn get_collection(collection_name: &str) -> mongodb::Collection {
        let db = MongoClint::conn().await;

        let collection = db.collection(collection_name);

        collection
    }
}

#[derive(Debug, Deserialize)]
pub struct Mongodb {
    pub host: String,
    pub db_name: String,
    pub password: String,
}

impl Mongodb {
    pub fn connection_string(&self) -> String {
        format!("mongodb+srv://PetHere:{}@{}", self.password, self.host)
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub mongodb: Mongodb,
}

impl Settings {
    pub fn new(base_path: PathBuf) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        let path = base_path.join("configuration/base");
        s.merge(File::from(path))?;

        // Detect the running environment
        let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        // Add in environment-specific settings (optional)
        let path = base_path.join(&format!("configuration/{}", environment));
        s.merge(File::from(path).required(false))?;

        // Add in settings from environment variables (with a prefix of APP and '_' as separator)
        // Eg.. `APP_APPLICATION_PORT=5001 would set `Settings.application.port`
        s.merge(Environment::with_prefix("app").separator("_"))?;

        // Deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
