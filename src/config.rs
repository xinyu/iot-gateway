extern crate toml;
extern crate serde;

use std::io;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(toml::de::Error),
}

#[derive(Deserialize)]
pub struct Settings {
    pub iothub: IOTHUB,
}

#[derive(Deserialize)]
pub struct IOTHUB {
    pub broker_address: String,
    pub thread_num: usize,
}

pub fn read_config<T: io::Read + Sized>(mut f: T) -> Result<Settings, ConfigError> {
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).map_err(ConfigError::Io)?;
    toml::from_str(&buffer).map_err(ConfigError::Parse)
}
