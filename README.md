# react-agent

A powerful, autonomous AI coding assistant built in Rust. This agent uses the **ReAct (Reasoning and Acting)** pattern to understand complex prompts, plan its actions, and execute them using a suite of integrated tools.

## 🚀 Capabilities

- **Autonomous Agent Loop**: Implements the ReAct pattern, allowing the agent to "think" (reason) and "act" (execute tools) in an iterative loop to solve multi-step problems.
- **Tool Integration**:
    - **💻 Bash Tool**: Execute shell commands directly to run scripts, install dependencies, or explore the system.
    - **📂 File Tool**: Read and write files within the project workspace, enabling autonomous coding and documentation.
- **LLM Agnostic**: Integrated with **OpenRouter**, allowing it to leverage various state-of-the-art models (like Claude 3.5 Sonnet or GPT-4o) while maintaining a consistent OpenAI-compatible interface.
- **Flexible Orchestration**: Handles context management, tool execution, and response parsing seamlessly.

## 📂 Project Structure

The project is organized into modular components for easy extensibility:

- **`src/agent/`**: The brain of the application. Contains the `Orchestrator` which manages the ReAct loop and maintains the conversation context.
- **`src/llm/`**: The communication layer. Handles requests to LLM providers (via OpenRouter) and parses responses into usable payloads.
- **`src/tools/`**: The agent's hands. Each tool (e.g., `BashTool`, `FileTool`) is implemented here as a discrete module with specific capabilities.
- **`src/main.rs`**: The main entry point that initializes the agent and processes user input.

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.92+)
- An OpenRouter API Key (set as an environment variable)

### Running the Agent

The recommended way to run the agent is using `cargo run`:

```bash
cargo run -- "Your prompt here"
```

## 📜 How it Works

1. **User Prompt**: You provide a task (e.g., "Implement a search function in src/lib.rs").
2. **Reasoning**: The agent analyzes the request and decides which tool to use.
3. **Action**: The agent executes a tool (like `read_file` or `bash`).
4. **Observation**: The agent receives the output of the tool.
5. **Iteration**: Steps 2-4 repeat until the agent determines the task is complete and provides a final response.
