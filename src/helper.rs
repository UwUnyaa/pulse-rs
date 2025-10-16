use std::process::{ChildStdin, ChildStdout, Command, Stdio};

pub fn spawn_helper() -> anyhow::Result<(ChildStdin, ChildStdout, std::thread::JoinHandle<()>)> {
    let binary_path = std::env::current_exe()?;
    let mut child = Command::new("pkexec")
        .arg(binary_path)
        .arg("--helper")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("Failed to open stdin for helper process"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("Failed to open stdout for helper process"))?;

    let handle = std::thread::spawn(move || {
        // TODO: implement
    });

    Ok((stdin, stdout, std::thread::JoinHandle::from(handle)))
}

pub fn helper_loop() -> i32 {
    // TODO: implement
    0
}
