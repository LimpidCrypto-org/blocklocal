use std::process::Command;

pub fn get_pid(name: &str) -> Option<u32> {
    let output = Command::new("pgrep")
        .arg(name)
        .output()
        .expect("Failed to execute pgrep");

    if output.status.success() {
        let pid = String::from_utf8(output.stdout).unwrap();
        Some(pid.trim().parse().unwrap())
    } else {
        None
    }
}
