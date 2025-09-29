// use std::path::Path; // TODO: ideally we would use a path

use std::process::Command;

pub fn rsync(
    local_path: &str,
    server_path: &str,
    server_user: &str,
    server_ip: &str,
    server_port: u16,
    bandwidth_limit_kbps: u32,
) {
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
        Err(err) => {
            eprintln!("could not call rsync: {err}");
            return;
        }
    };

    if !cmd.status.success() {
        eprintln!(
            "rsync failure [{}]:\nvvvvv\n{}\n^^^^^",
            cmd.status,
            String::from_utf8_lossy(&cmd.stderr)
        );
        return;
    }
}
