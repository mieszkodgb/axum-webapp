use crate::{Error, Result};
use std::{env, sync::OnceLock};

// To load the config only once at start and panic if missing
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            panic!("ERROR WHILE LOADING THE CONFIG DUE TO : {:?}", err)
        })
    })
}

#[allow(non_snake_case)]
pub struct Config{
    pub WEB_FOLDER: String
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config{
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
