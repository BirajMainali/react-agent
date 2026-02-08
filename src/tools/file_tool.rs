use serde::Deserialize;
use std::path::Path;

pub struct FileTool;

impl FileTool {
    pub const READ_NAME: &'static str = "Read";
    pub const WRITE_NAME: &'static str = "Write";

    pub fn read_file(args_json: &str) -> Result<String, Box<dyn std::error::Error>> {
        let args: ReadFileToolArgs = serde_json::from_str(args_json)?;
        let path = Path::new(&args.file_path);
        let content = std::fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn write_file(args_json: &str) -> Result<(), Box<dyn std::error::Error>> {
        let args: WriteFileToolArgs = serde_json::from_str(args_json)?;
        let path = Path::new(&args.file_path);
        let content = args.content;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct ReadFileToolArgs {
    pub file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct WriteFileToolArgs {
    pub file_path: String,
    pub content: String,
}
