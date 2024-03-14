fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_hello_world_output() {
        let output = Command::new("cargo")
                             .arg("run")
                             .arg("--quiet")
                             .output()
                             .expect("Failed to run command");

        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");

        assert!(output.status.success(), "Command failed with: {}", stderr);

        assert_eq!(stdout.trim(), "Hello, world!");
    }
}