// use std::path::Path; // TODO: ideally we would use a path

use std::process::Command;

pub fn rsync(
    local_path: &str,
    server_path: &str,
    server_user: &str,
    server_ip: &str,
    server_port: u16,
    bandwidth_limit_kbps: u32,
) -> Result<(), String> {
    // TODO capturing the output like that is not OK

    let cmd = match Command::new("rsync")
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
        .output()
    {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("call to rsync failed: {e}")); // TODO: ugly, 100% there is a better way
        }
    };

    if !cmd.status.success() {
        return Err(format!(
            "rsync failure [{}]:\n{}",
            cmd.status,
            String::from_utf8_lossy(&cmd.stderr)
        ));
    };

    Ok(())
}
