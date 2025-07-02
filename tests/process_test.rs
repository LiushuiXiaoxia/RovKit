#[cfg(test)]
mod tests {
    use rovkit::processkit::{call, run, run_and_get_stdout};

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
