use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::process::{ChildStdin, ChildStdout};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::{io, io::BufRead, io::BufReader, io::Write};

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

pub struct HelperIO {
    pub stdin: ChildStdin,
    pub stdout: BufReader<ChildStdout>,
}

pub type HelperIORef = Rc<RefCell<HelperIO>>;

pub fn spawn_helper() -> anyhow::Result<HelperIORef> {
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

    Ok(Rc::new(RefCell::new(HelperIO {
        stdin: child_stdin,
        stdout: BufReader::new(child_stdout),
    })))
}

fn send_helper_response(out: &mut impl Write, response: &HelperResponse) -> anyhow::Result<()> {
    let response_str = match serde_json::to_string(response) {
        Ok(s) => s,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to serialize helper response: {}",
                e
            ));
        }
    };

    writeln!(out, "{}", response_str)?;

    Ok(())
}

pub fn helper_loop() -> i32 {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = std::io::BufReader::new(stdin);
    let mut out = stdout.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to read line from stdin: {}", e);
                continue;
            }
        };

        let req: HelperRequest = match serde_json::from_str(&line) {
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
                    let _ = send_helper_response(&mut out, &HelperResponse::Error(e.to_string()));
                } else {
                    let _ = send_helper_response(&mut out, &HelperResponse::Ok);
                }
            }
            HelperRequest::Ping => {
                let _ = send_helper_response(&mut out, &HelperResponse::Pong);
            }
        }
    }

    0
}

pub fn send_helper_request(
    helper_io: HelperIORef,
    request: &HelperRequest,
) -> anyhow::Result<HelperResponse> {
    let mut io = helper_io.borrow_mut();
    let request_str = serde_json::to_string(request)?;
    writeln!(io.stdin, "{}", request_str)?;

    let mut response_line = String::new();
    io.stdout.read_line(&mut response_line)?;

    let response: HelperResponse = serde_json::from_str(&response_line)?;

    Ok(response)
}
