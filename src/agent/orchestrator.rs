use crate::llm::{ApiResponse, MessagePayload, OpenRouterClient};
use crate::tools::bash_tool::BashTool;
use crate::tools::file_tool::FileTool;
use async_openai::types::chat::Role;

pub struct Orchestrator {
    client: OpenRouterClient,
    max_iterations: usize,
}

impl Orchestrator {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            client: OpenRouterClient::new(),
            max_iterations,
        }
    }

    pub async fn run(&self, prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut context: Vec<MessagePayload> = vec![MessagePayload {
            role: Role::User,
            content: Some(prompt),
            tool_calls: None,
            tool_call_id: None,
        }];

        let mut iteration = 0;

        loop {
            if self.max_iterations <= iteration {
                println!(
                    "Reached maximum loop count ({}) without finishing.",
                    self.max_iterations
                );
                return Ok(());
            }

            let response = self.client.chat(&context).await?;
            self.build_agent_loop_context(&mut context, &response)?;

            if response.finished_with_stop() {
                if let Some(output) = response.content() {
                    println!("{}", output.trim_start_matches('\n'));
                }
                return Ok(());
            }

            iteration += 1;
        }
    }

    fn build_agent_loop_context(
        &self,
        context: &mut Vec<MessagePayload>,
        response: &ApiResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Find the first choice to get the assistant's message
        if let Some(choice) = response.choices.first() {
            let msg = &choice.message;

            // Push the Assistant's message (which contains the tool calls or content)
            context.push(MessagePayload {
                role: Role::Assistant,
                content: msg.content.clone(),
                tool_calls: msg.tool_calls.clone(),
                tool_call_id: None,
            });

            // If there are tool calls, execute them and push Tool messages
            if let Some(tool_calls) = &msg.tool_calls {
                for tool in tool_calls {
                    match tool.function.name.as_str() {
                        FileTool::READ_NAME => {
                            let file_content = FileTool::read_file(&tool.function.arguments)?;
                            context.push(MessagePayload {
                                content: Some(if file_content.is_empty() {
                                    "The file has no content".to_string()
                                } else {
                                    file_content
                                }),
                                role: Role::Tool,
                                tool_calls: None,
                                tool_call_id: Some(tool.id.clone()),
                            });
                        }
                        FileTool::WRITE_NAME => {
                            FileTool::write_file(&tool.function.arguments)?;
                            context.push(MessagePayload {
                                content: response.content(),
                                role: Role::Tool,
                                tool_calls: None,
                                tool_call_id: Some(tool.id.clone()),
                            });
                        }
                        BashTool::NAME => {
                            let output = BashTool::run(&tool.function.arguments)?;
                            context.push(MessagePayload {
                                content: Some(output),
                                role: Role::Tool,
                                tool_calls: None,
                                tool_call_id: Some(tool.id.clone()),
                            });
                        }
                        _ => println!("Unknown tool: {}", tool.function.name),
                    }
                }
            }
        }

        Ok(())
    }
}
