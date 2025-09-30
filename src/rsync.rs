// use std::path::Path; // TODO: ideally we would use a path

use std::process::{Command, Stdio};

pub fn rsync(
    local_path: &str,
    server_path: &str,
    server_user: &str,
    server_ip: &str,
    server_port: u16,
    bandwidth_limit_kbps: u32,
) -> Result<(), String> {
    // TODO: this DOES NOT DIE if the gui is closed
    let mut child = Command::new("rsync")
        .args([
            "-av",
            "--delete-after",
            "--info=progress2",
            &format!("--bwlimit={bandwidth_limit_kbps}"),
            "-e",
            &format!("ssh -p {server_port}"),
            local_path,
            &format!("{server_user}@{server_ip}:{server_path}"),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("call to rsync failed: {e}"))?;

    let exit_status = child
        .wait()
        .map_err(|e| format!("could not wait for rsync: {e}"))?;

    // TODO: ignore the acceptable reutrn codes - see `man rsync` - `EXIT VALUES` - `0 - Success`
    if !exit_status.success() {
        return Err(format!(
            "rsync bad exit status (see the terminal output for more info)\n{exit_status}",
        ));
    };

    Ok(())
}
