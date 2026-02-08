use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,

    #[serde(rename = "type")]
    pub call_type: String,

    pub function: FunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

impl ApiResponse {
    pub fn is_tool_call(&self) -> bool {
        self.choices
            .iter()
            .any(|choice| choice.message.tool_calls.is_some())
    }

    pub fn tool_calls(&self) -> Option<Vec<ToolCall>> {
        self.choices
            .iter()
            .find_map(|choice| choice.message.tool_calls.clone())
    }

    pub fn finished_with_stop(&self) -> bool {
        self.choices
            .iter()
            .any(|choice| choice.finish_reason.trim().to_lowercase() == "stop")
    }

    pub fn content(&self) -> Option<String> {
        self.choices
            .iter()
            .find_map(|choice| choice.message.content.clone())
    }
}
