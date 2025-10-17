use serde::{Deserialize, Serialize};
use std::process::{ChildStdin, ChildStdout};
use std::process::{Command, Stdio};
use std::{io, io::BufRead, io::Write};

#[derive(Serialize, Deserialize, Debug)]
pub enum HelperRequest {
    SetCPUEnableState { cpu_num: u32, enabled: bool },
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HelperResponse {
    Ok,
    Error(String),
    Pong,
}

pub fn spawn_helper() -> anyhow::Result<(ChildStdin, ChildStdout)> {
    let binary_path = std::env::current_exe()?;
    let mut child = Command::new("pkexec")
        .arg(binary_path)
        .arg("--helper")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("failed to take child stdin"))?;

    let child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("failed to take child stdout"))?;

    Ok((child_stdin, child_stdout))
}

pub fn helper_loop() -> i32 {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = std::io::BufReader::new(stdin);
    let mut out = stdout.lock();

    for line in reader.lines() {
        let req: HelperRequest = match serde_json::from_str(&line.unwrap()) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Failed to parse helper request: {}", e);
                continue;
            }
        };
        match req {
            HelperRequest::SetCPUEnableState { cpu_num, enabled } => {
                let path = format!("/sys/devices/system/cpu/cpu{}/online", cpu_num);
                if let Err(e) = std::fs::write(&path, if enabled { "1" } else { "0" }) {
                    eprintln!("Failed to set CPU {} state: {}", cpu_num, e);
                    let _ =
                        serde_json::to_string(&HelperResponse::Error(e.to_string())).map(|resp| {
                            writeln!(out, "{}", resp).unwrap();
                        });
                    continue;
                }
                let _ = serde_json::to_string(&HelperResponse::Ok).map(|resp| {
                    writeln!(out, "{}", resp).unwrap();
                });
            }
            HelperRequest::Ping => {
                let _ = serde_json::to_string(&HelperResponse::Pong).map(|resp| {
                    writeln!(out, "{}", resp).unwrap();
                });
            }
        }
    }

    0
}
