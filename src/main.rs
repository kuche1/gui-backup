mod rsync;

use dirs; // cargo add dirs
use serde::Deserialize; // cargo add serde --features derive
use std::fs;
use toml; // cargo add toml

use crate::rsync::rsync;

// TODO: put in a separate file
#[derive(Deserialize)]
struct Config {
    local_path: String,
    server_path: String,
    server_user: String,
    server_ip: String,
    server_port: u16,
    bandwidth_limit_kbps: u32,
}

fn main() {
    let mut config_path = dirs::config_dir().unwrap(); // TODO: GUI error message
    config_path.push("gui-backup/config.toml");

    let content = fs::read_to_string(config_path).unwrap();

    let config: Config = toml::from_str(&content).unwrap(); // TODO: GUI error message

    rsync(
        &config.local_path,
        &config.server_path,
        &config.server_user,
        &config.server_ip,
        config.server_port,
        config.bandwidth_limit_kbps,
    );
}
