use std::fs::File;
use serde::Deserialize;

pub fn try_read_config() -> Config {
    let fs = File::open("config.yaml").expect("Failed to open config.yml");
    let config = serde_yaml::from_reader(fs).expect("Invalid config format");

    config
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub user_service: UserServiceConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserServiceConfig {
    pub persistence: PersistenceDriverSelection,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "driver", content = "config", rename_all = "snake_case")]
pub enum PersistenceDriverSelection {
    InMemory,
    Postgres(PostgresPersistenceDiverConfig),
}
#[derive(Deserialize, Clone, Debug)]
pub struct PostgresPersistenceDiverConfig {
    pub connection_string: String,
    pub table: String,
}
