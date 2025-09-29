use crate::config::Config;
use crate::rsync::rsync;

pub fn do_work() -> Result<(), String> {
    let config = Config::read()?;

    rsync(
        &config.local_path,
        &config.server_path,
        &config.server_user,
        &config.server_ip,
        config.server_port,
        config.bandwidth_limit_kbps,
    )?;

    Ok(())
}
