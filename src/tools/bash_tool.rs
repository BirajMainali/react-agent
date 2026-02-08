use serde::Deserialize;
use std::process::Command;

pub struct BashTool;

impl BashTool {
    pub const NAME: &'static str = "Bash";

    pub fn run(args_json: &str) -> Result<String, Box<dyn std::error::Error>> {
        let args: BashToolArgs = serde_json::from_str(args_json)?;
        let output = Command::new("sh").arg("-c").arg(&args.command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let result = if output.status.success() {
            format!("{}{}", stdout, stderr)
        } else {
            format!(
                "Command failed with exit code {:?}\nstdout: {}\nstderr: {}",
                output.status.code(),
                stdout,
                stderr
            )
        };

        Ok(result)
    }
}

#[derive(Deserialize, Debug)]
pub struct BashToolArgs {
    pub command: String,
}
