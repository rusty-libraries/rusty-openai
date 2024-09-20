# OpenAI Rust SDK Documentation

Welcome to the OpenAI Rust SDK documentation. This SDK allows you to interact with OpenAI's API endpoints to perform various tasks such as generating completions, creating images, moderating text, etc.

## Getting Started

Before using the SDK, make sure you have the following dependencies added to your `Cargo.toml` file:

```toml
[dependencies]
rusty_openai = "0.1.5"
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12.5", features = ["json", "multipart"] }
```

## Usage

First, set up the necessary imports:

```rust
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::completion::ChatCompletionRequest;
use serde_json::json;
```

### Initialize OpenAI Client

Create an instance of OpenAI with your API key:

```rust
#[tokio::main]
async fn main() {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");
}
```

### Generate Chat Completions

To generate chat completions, create a `ChatCompletionRequest` object and call the `create` method from the completions API:

```rust
#[tokio::main]
async fn main() {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let messages = vec![
        json!({
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Hello"
                },
            ]
        })
    ];

    let request = ChatCompletionRequest::new("gpt-4".to_string(), messages)
        .max_tokens(300)
        .temperature(0.7);

    let chat_response = openai.completions().create(request).await;

    match chat_response {
        Ok(chat) => println!("{}", json!(chat).to_string()),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

### Endpoints Documentation

#### Completions API

**Generate Chat Completions**

Use the provided `ChatCompletionRequest` to create chat completions:

```rust
pub async fn create(&self, request: ChatCompletionRequest) -> OpenAIResult<Value>
```

- **Parameters:**
  - `request`: `ChatCompletionRequest`
    - `model`: The model name to be used for the chat completion.
    - `messages`: The history of messages in the conversation.
    - Fluent setter methods for additional options like `max_tokens`, `temperature`, etc.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


#### Assistants API

**Create An Assistant**

Create an assistant with the specified parameters:

```rust
pub async fn create(&self, request: AssistantRequest) -> OpenAIResult<Value>
```

- **Parameters:**
  - `request`: `AssistantRequest`
    - Contains fields like `model`, `name`, `description`, `instructions`, `tools`, `temperature`, etc.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**List Assistants**

List all assistants:

```rust
pub async fn list(&self, limit: Option<u32>, order: Option<&str>, after: Option<&str>, before: Option<&str>) -> OpenAIResult<Value>
```

- **Parameters:**
  - `limit`: Optional maximum number of assistants to retrieve.
  - `order`: Optional order to return the results.
  - `after`: Optional cursor to use for pagination.
  - `before`: Optional cursor to use for pagination.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Retrieve Assistant**

Retrieve information about a specific assistant:

```rust
pub async fn retrieve(&self, assistant_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `assistant_id`: The ID of the assistant to retrieve.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Modify Assistant**

Modify an existing assistant with the provided parameters:

```rust
pub async fn modify(&self, assistant_id: &str, request: AssistantRequest) -> OpenAIResult<Value>
```

- **Parameters:**
  - `assistant_id`: The ID of the assistant to modify.
  - `request`: `AssistantRequest`
    - Contains fields like `name`, `description`, `instructions`, `tools`, `temperature`, etc.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Delete Assistant**

Delete an assistant using the provided ID:

```rust
pub async fn delete(&self, assistant_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `assistant_id`: The ID of the assistant to delete.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


#### Threads API

**Create A Thread**

Create a new conversation thread with the specified parameters:

```rust
pub async fn create(&self, request: ThreadRequest) -> OpenAIResult<Value>
```

- **Parameters:**
  - `request`: `ThreadRequest`
    - Contains fields like `messages`, `tool_resources`, and `metadata`.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Retrieve Thread**

Retrieve information about a specific thread:

```rust
pub async fn retrieve(&self, thread_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to retrieve.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Modify Thread**

Modify an existing thread with the provided parameters:

```rust
pub async fn modify(&self, thread_id: &str, request: ThreadRequest) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to modify.
  - `request`: `ThreadRequest`
    - Contains fields like `tool_resources` and `metadata`.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Delete Thread**

Delete a thread using the provided ID:

```rust
pub async fn delete(&self, thread_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to delete.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Create Message**

Add a message to an existing thread:

```rust
pub async fn create_message(&self, thread_id: &str, role: &str, content: Value, attachments: Option<Value>, metadata: Option<Value>) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to add a message to.
  - `role`: The role of the sender of the message.
  - `content`: The content of the message.
  - `attachments`: Optional attachments for the message.
  - `metadata`: Optional metadata for the message.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**List Messages**

List all messages in a thread:

```rust
pub async fn list_messages(&self, thread_id: &str, limit: Option<u32>, order: Option<&str>, after: Option<&str>, before: Option<&str>) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to list messages from.
  - `limit`: Optional maximum number of messages to retrieve.
  - `order`: Optional order to return the results.
  - `after`: Optional cursor to use for pagination.
  - `before`: Optional cursor to use for pagination.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Retrieve Message**

Retrieve a specific message from a thread:

```rust
pub async fn retrieve_message(&self, thread_id: &str, message_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to retrieve the message from.
  - `message_id`: The ID of the message to retrieve.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Modify Message**

Modify an existing message in a thread:

```rust
pub async fn modify_message(&self, thread_id: &str, message_id: &str, metadata: Value) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to modify the message in.
  - `message_id`: The ID of the message to modify.
  - `metadata`: The metadata to update in the message.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Delete Message**

Delete a specific message from a thread:

```rust
pub async fn delete_message(&self, thread_id: &str, message_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to delete the message from.
  - `message_id`: The ID of the message to delete.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Create Run**

Create a run for a specific assistant in a thread:

```rust
pub async fn create_run(
    &self,
    thread_id: &str,
    assistant_id: &str,
    model: Option<&str>,
    instructions: Option<&str>,
    additional_instructions: Option<&str>,
    additional_messages: Option<Vec<Value>>,
    tools: Option<Vec<Value>>,
    metadata: Option<Value>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    stream: Option<bool>,
    max_prompt_tokens: Option<u32>,
    max_completion_tokens: Option<u32>,
    truncation_strategy: Option<Value>,
    tool_choice: Option<Value>,
    parallel_tool_calls: Option<bool>,
    response_format: Option<Value>,
) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to create the run in.
  - `assistant_id`: The ID of the assistant to use for the run.
  - Optional parameters including `model`, `instructions`, `additional_instructions`, `additional_messages`, `tools`, `metadata`, `temperature`, `top_p`, `stream`, `max_prompt_tokens`, `max_completion_tokens`, `truncation_strategy`, `tool_choice`, `parallel_tool_calls`, and `response_format`.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**List Runs**

List all runs in a thread:

```rust
pub async fn list_runs(&self, thread_id: &str, limit: Option<u32>, order: Option<&str>, after: Option<&str>, before: Option<&str>) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to list runs from.
  - `limit`: Optional maximum number of runs to retrieve.
  - `order`: Optional order to return the results.
  - `after`: Optional cursor to use for pagination.
  - `before`: Optional cursor to use for pagination.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Retrieve Run**

Retrieve a specific run from a thread:

```rust
pub async fn retrieve_run(&self, thread_id: &str, run_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to retrieve the run from.
  - `run_id`: The ID of the run to retrieve.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Modify Run**

Modify an existing run in a thread:

```rust
pub async fn modify_run(&self, thread_id: &str, run_id: &str, metadata: Value) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to modify the run in.
  - `run_id`: The ID of the run to modify.
  - `metadata`: The metadata to update in the run.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Delete Run**

Delete a specific run from a thread:

```rust
pub async fn delete_run(&self, thread_id: &str, run_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to delete the run from.
  - `run_id`: The ID of the run to delete.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Submit Tool Outputs**

Submit tool outputs for a run in a thread:

```rust
pub async fn submit_tool_outputs(&self, thread_id: &str, run_id: &str, tool_outputs: Vec<Value>, stream: Option<bool>) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to submit tool outputs for.
  - `run_id`: The ID of the run to submit tool outputs for.
  - `tool_outputs`: The tool outputs to submit.
  - `stream`: Optional flag to stream the tool outputs.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Cancel Run**

Cancel a specific run in a thread:

```rust
pub async fn cancel_run(&self, thread_id: &str, run_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to cancel the run in.
  - `run_id`: The ID of the run to cancel.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**List Run Steps**

List all steps of a specific run in a thread:

```rust
pub async fn list_run_steps(&self, thread_id: &str, run_id: &str, limit: Option<u32>, order: Option<&str>, after: Option<&str>, before: Option<&str>) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to list run steps from.
  - `run_id`: The ID of the run to list steps from.
  - `limit`: Optional maximum number of steps to retrieve.
  - `order`: Optional order to return the results.
  - `after`: Optional cursor to use for pagination.
  - `before`: Optional cursor to use for pagination.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Retrieve Run Step**

Retrieve a specific step from a run in a thread:

```rust
pub async fn retrieve_run_step(&self, thread_id: &str, run_id: &str, step_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `thread_id`: The ID of the thread to retrieve the run step from.
  - `run_id`: The ID of the run to retrieve the step from.
  - `step_id`: The ID of the step to retrieve.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


#### Vectors API

**Create Vector Store**

Create a vector store with the specified parameters:

```rust
pub async fn create_vector_store(&self, request: VectorStoreRequest) -> OpenAIResult<Value>
```

- **Parameters:**
  - `request`: `VectorStoreRequest`
    - Contains fields like `file_ids`, `name`, `expires_after`, `chunking_strategy`, and `metadata`.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**List Vector Stores**

List all vector stores:

```rust
pub async fn list_vector_stores(&self, limit: Option<u64>, order: Option<String >, after: Option<String>, before: Option<String>) -> OpenAIResult<Value>
```

- **Parameters:**
  - `limit`: Optional maximum number of vector stores to retrieve.
  - `order`: Optional order to return the results.
  - `after`: Optional cursor to use for pagination.
  - `before`: Optional cursor to use for pagination.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Retrieve Vector Store**

Retrieve information about a specific vector store:

```rust
pub async fn retrieve_vector_store(&self, vector_store_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `vector_store_id`: The ID of the vector store to retrieve.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Modify Vector Store**

Modify an existing vector store with the provided parameters:

```rust
pub async fn modify_vector_store(&self, vector_store_id: &str, request: VectorStoreRequest) -> OpenAIResult<Value>
```

- **Parameters:**
  - `vector_store_id`: The ID of the vector store to modify.
  - `request`: `VectorStoreRequest`
    - Contains fields like `name`, `expires_after`, and `metadata`.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.

**Delete Vector Store**

Delete a vector store using the provided ID:

```rust
pub async fn delete_vector_store(&self, vector_store_id: &str) -> OpenAIResult<Value>
```

- **Parameters:**
  - `vector_store_id`: The ID of the vector store to delete.

- **Returns:**
  - `OpenAIResult<Value>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.