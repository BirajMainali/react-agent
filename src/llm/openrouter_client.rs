use crate::{
    llm::openai_models::ApiResponse,
    tools::{bash_tool::BashTool, file_tool::FileTool},
};
use async_openai::{Client, config::OpenAIConfig, types::chat::Role};
use serde::Serialize;
use serde_json::{Value, json};
use std::{env, process};

pub struct OpenRouterClient {
    base_url: String,
    api_key: String,
}

impl OpenRouterClient {
    pub fn new() -> Self {
        let base_url = env::var("OPENROUTER_BASE_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

        let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
            eprintln!("OPENROUTER_API_KEY is not set");
            process::exit(1);
        });

        OpenRouterClient { base_url, api_key }
    }

    pub async fn chat(&self, context: &Vec<MessagePayload>) -> anyhow::Result<ApiResponse> {
        let config = OpenAIConfig::new()
            .with_api_base(self.base_url.clone())
            .with_api_key(self.api_key.clone());

        let client = Client::with_config(config);
        let messages = serde_json::to_value(&context).unwrap();

        let request = json!({
            "model": "anthropic/claude-haiku-4.5",
            "messages": messages,
            "tools": [{
                "type": "function",
                "function": {
                    "name": FileTool::READ_NAME,
                    "description": "Read and return the contents of a file",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "The path to the file to read"
                            }
                        },
                        "required": ["file_path"]
                    }
                }
            },
            {
              "type": "function",
              "function": {
                "name": FileTool::WRITE_NAME,
                "description": "Write content to a file",
                "parameters": {
                  "type": "object",
                  "required": ["file_path", "content"],
                  "properties": {
                    "file_path": {
                      "type": "string",
                      "description": "The path of the file to write to"
                    },
                    "content": {
                      "type": "string",
                      "description": "The content to write to the file"
                    }
                  }
                }
              }
            },
            {
              "type": "function",
              "function": {
                "name": BashTool::NAME,
                "description": "Execute a shell command",
                "parameters": {
                  "type": "object",
                  "required": ["command"],
                  "properties": {
                    "command": {
                      "type": "string",
                      "description": "The command to execute"
                    }
                  }
                }
              }
            }]
        });

        let response: Value = client.chat().create_byot(request).await?;
        let parsed: ApiResponse = serde_json::from_value(response)?;
        Ok(parsed)
    }
}

#[derive(Serialize)]
pub struct MessagePayload {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "role")]
    pub role: Role,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<crate::llm::openai_models::ToolCall>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}
