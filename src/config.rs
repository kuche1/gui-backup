use serde::Deserialize; // cargo add serde --features derive
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub local_path: String,
    pub server_path: String,
    pub server_user: String,
    pub server_ip: String,
    pub server_port: u16,
    pub bandwidth_limit_kbps: u32,
}

impl Config {
    pub fn read() -> Result<Self, String> {
        let mut config_path = dirs::config_dir().ok_or("could not find config dir")?;
        config_path.push("gui-backup/config.toml");

        let content = fs::read_to_string(&config_path).map_err(|_e| {
            format!(
                "конфигурационният файл не съществува: {}",
                config_path.display()
            )
        })?;

        let config: Config =
            toml::from_str(&content).map_err(|_e| "could not parse config file")?;

        Ok(config)
    }
}
