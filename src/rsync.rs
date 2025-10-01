// use std::path::Path; // TODO: ideally we would use a path

use libc::{PR_SET_PDEATHSIG, SIGTERM, prctl}; // cargo add libc
use std::io;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

pub fn rsync(
    local_path: &str,
    server_path: &str,
    server_user: &str,
    server_ip: &str,
    server_port: u16,
    bandwidth_limit_kbps: u32,
) -> Result<(), String> {
    let mut cmd = Command::new("rsync");

    cmd.args([
        "-av",
        "--delete-after",
        "--info=progress2",
        &format!("--bwlimit={bandwidth_limit_kbps}"),
        "-e",
        &format!("ssh -p {server_port}"),
        local_path,
        &format!("{server_user}@{server_ip}:{server_path}"),
    ]);

    // redirect output to terminal
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    // make the child process terminate if the parent dies
    unsafe {
        cmd.pre_exec(|| {
            let ret = prctl(PR_SET_PDEATHSIG, SIGTERM);
            if ret != 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    };

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("call to rsync failed: {e}"))?;

    let exit_status = child
        .wait()
        .map_err(|e| format!("could not wait for rsync: {e}"))?;

    if !exit_status.success() {
        let code = exit_status
            .code()
            .ok_or("rsync process was killed by a signal")?;

        // add any "ok" return codes here
        // see `man rsync` - `EXIT VALUES` - `0 - Success`

        if code == 24 {
            // 24 - Partial transfer due to vanished source files
            return Ok(());
        }

        return Err(format!("rsync грешка # {code}"));
    };

    Ok(())
}
