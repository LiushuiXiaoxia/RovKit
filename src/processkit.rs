use std::io;
use std::process::{Command, Output, Stdio};
#[cfg(target_os = "windows")]
const SHELL: &str = "cmd";
#[cfg(target_os = "windows")]
const SHELL_FLAG: &str = "/C";

#[cfg(not(target_os = "windows"))]
const SHELL: &str = "bash";
#[cfg(not(target_os = "windows"))]
const SHELL_FLAG: &str = "-c";

/// 同步运行命令字符串并将 stdout 打印到控制台（实时输出）
pub fn run(cmd: &str) -> io::Result<()> {
    let mut child = Command::new(SHELL)
        .arg(SHELL_FLAG)
        .arg(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;
    if !status.success() {
        eprintln!("Command exited with: {}", status);
    }
    Ok(())
}

/// 获取命令输出作为字符串
pub fn run_and_get_stdout(cmd: &str) -> io::Result<String> {
    let output = call(cmd)?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// 运行命令字符串并捕获输出（stdout + stderr）
pub fn call(command: &str) -> io::Result<Output> {
    Command::new(SHELL)
        .arg(SHELL_FLAG)
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run("cd src && ls -alh").unwrap()
    }
    #[test]
    fn test_call() {
        let output = call("echo hello").unwrap();
        let out_str = String::from_utf8_lossy(&output.stdout);
        assert!(out_str.contains("hello"));
    }

    #[test]
    fn test_run_and_get_stdout() {
        let result = run_and_get_stdout("echo world").unwrap();
        assert!(result.contains("world"));
    }
}
