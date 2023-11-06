use std::path::PathBuf;

const DEFAULT_CONFIG_DIR: &str = "cl.toml";

pub struct Configurations {
    pub network: NetworkSettings,
}

pub struct NetworkSettings {
    pub addr: String,
    pub port: String,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".to_string(),
            port: "6969".to_string(),
        }
    }
}

impl Configurations {
    pub fn build() -> Self {
        let config_dir = PathBuf::from(DEFAULT_CONFIG_DIR);

        if config_dir.try_exists().unwrap() && config_dir.is_file() {
            todo!()
        } else {
            Self {
                network: NetworkSettings::default(),
            }
        }
    }
}
