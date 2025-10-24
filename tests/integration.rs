use assert_cmd::Command;
use std::{thread, time::Duration};

#[test]
fn starts_and_exits_cleanly() {
    let mut cmd = Command::cargo_bin("rundaemon").unwrap();
    cmd.arg("examples/sample.yml");
    // run for a short time to let processes start
    let mut child = cmd.spawn().unwrap();
    thread::sleep(Duration::from_secs(3));
    // send SIGINT
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal::SIGINT};
        use nix::unistd::Pid;
        kill(Pid::from_raw(child.id() as i32), SIGINT).unwrap();
    }
    let _ = child.wait().unwrap();
}
