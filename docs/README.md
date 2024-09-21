## OpenAI Rust SDK - Function Documentation

### Completions API

#### Create a Chat Completion

**Functionality:**  
Generate the next message in a conversational exchange by sending a structured list of input messages containing text and/or image content. Supports both single queries and stateless multi-turn conversations.

**Method:**  
POST `/v1/chat/completions`

**Parameters:**

- **model (string, required):**  
  The model to use for generating the completion (e.g., "gpt-4").

- **messages (Vec<Value>, required):**  
  A list of messages representing the conversation history. Each message should specify a role (`"user"` or `"assistant"`) and its content.

- **max_tokens (Option<u64>):**  
  The maximum number of tokens to generate in the completion.

- **temperature (Option<f64>):**  
  Controls the randomness of the output. Higher values like 0.8 make the output more random, while lower values like 0.2 make it more focused and deterministic. Default is 1.0.

- **top_p (Option<f64>):**  
  Controls diversity via nucleus sampling. The model considers the smallest possible set of tokens with a cumulative probability above `top_p`. Default is 1.0.

- **frequency_penalty (Option<f64>):**  
  How much to penalize new tokens based on their existing frequency in the text so far. Values range from -2.0 to 2.0. Default is 0.

- **presence_penalty (Option<f64>):**  
  How much to penalize new tokens based on whether they appear in the text so far. Values range from -2.0 to 2.0. Default is 0.

- **stop_sequences (Option<Vec<String>>):**  
  Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.

- **stream (Option<bool>):**  
  If set to `true`, partial message deltas will be sent as data-only server-sent events.

**Response:**

- **id (string):**  
  Unique identifier for the completion.

- **object (string):**  
  The object type, typically `"chat.completion"`.

- **created (u64):**  
  Timestamp of when the completion was created.

- **choices (Vec<Choice>):**  
  Array of generated completions. Each choice contains:
  - **message (Message):**  
    The generated message.
  - **finish_reason (string):**  
    The reason the completion finished (`"stop"`, `"length"`, etc.).
  - **index (u32):**  
    The index of the choice.

- **usage (Usage):**  
  Information about token usage.

**Usage Example:**

```rust
use serde_json::json;
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::completion::ChatCompletionRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let messages = vec![
        json!({
            "role": "user",
            "content": "Hello, how are you?"
        }),
        json!({
            "role": "assistant",
            "content": "I'm good, thank you! How can I assist you today?"
        }),
    ];

    let request = ChatCompletionRequest::new("gpt-4".to_string(), messages)
        .max_tokens(150)
        .temperature(0.7)
        .stop_sequences(vec!["\n".to_string()]);

    let response = openai.completions().create(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

### Text Completions API

#### Create a Text Completion

**Functionality:**  
Generate a text completion based on a provided prompt, allowing for control over various aspects of the completion process.

**Method:**  
POST `/v1/completions`

**Parameters:**

- **model (string, required):**  
  The model to use for generating the completion (e.g., "text-davinci-003").

- **prompt (string or Vec<string>, required):**  
  The input text prompt(s) to generate completions for.

- **max_tokens (Option<u64>):**  
  The maximum number of tokens to generate in the completion.

- **temperature (Option<f64>):**  
  Controls the randomness of the output. Higher values like 0.8 make the output more random, while lower values like 0.2 make it more focused and deterministic. Default is 1.0.

- **top_p (Option<f64>):**  
  Controls diversity via nucleus sampling. The model considers the smallest possible set of tokens with a cumulative probability above `top_p`. Default is 1.0.

- **frequency_penalty (Option<f64>):**  
  How much to penalize new tokens based on their existing frequency in the text so far. Values range from -2.0 to 2.0. Default is 0.

- **presence_penalty (Option<f64>):**  
  How much to penalize new tokens based on whether they appear in the text so far. Values range from -2.0 to 2.0. Default is 0.

- **stop_sequences (Option<Vec<String>>):**  
  Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.

- **n (Option<u32>):**  
  Number of completions to generate for each prompt.

**Response:**

- **id (string):**  
  Unique identifier for the completion.

- **object (string):**  
  The object type, typically `"text_completion"`.

- **created (u64):**  
  Timestamp of when the completion was created.

- **choices (Vec<Choice>):**  
  Array of generated completions. Each choice contains:
  - **text (string):**  
    The generated completion text.
  - **index (u32):**  
    The index of the choice.
  - **logprobs (Option<Logprobs>):**  
    Log probabilities of the tokens (if requested).
  - **finish_reason (string):**  
    The reason the completion finished (`"stop"`, `"length"`, etc.).

- **usage (Usage):**  
  Information about token usage.

**Usage Example:**

```rust
use serde_json::json;
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::completion::TextCompletionRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let prompt = "Once upon a time in a land far, far away";

    let request = TextCompletionRequest::new("text-davinci-003".to_string(), prompt.to_string())
        .max_tokens(100)
        .temperature(0.6)
        .stop_sequences(vec!["The end.".to_string()]);

    let response = openai.completions().create_text_completion(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

### Embeddings API

#### Create Embeddings

**Functionality:**  
Generate embeddings for a list of input texts, useful for tasks like search, recommendations, and anomaly detection.

**Method:**  
POST `/v1/embeddings`

**Parameters:**

- **model (string, required):**  
  The model to use for generating embeddings (e.g., "text-embedding-ada-002").

- **inputs (Vec<String>, required):**  
  List of input texts to generate embeddings for.

- **input_type (Option<String>):**  
  The type of input text (e.g., `"query"`, `"document"`).

- **user (Option<String>):**  
  A unique identifier representing the user, which can help OpenAI monitor and detect abuse.

**Response:**

- **data (Vec<EmbeddingData>):**  
  Array of embeddings corresponding to each input text. Each embedding contains:
  - **embedding (Vec<f64>):**  
    The embedding vector.
  - **index (u32):**  
    The index of the input text.
  - **object (string):**  
    The object type, typically `"embedding"`.

- **model (string):**  
  The model used to generate the embeddings.

- **usage (Usage):**  
  Information about token usage.

**Usage Example:**

```rust
use serde_json::json;
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::embedding::EmbeddingsRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let inputs = vec![
        "OpenAI develops and promotes friendly AI for the benefit of all humanity.".to_string(),
        "Rust is a systems programming language focused on safety and performance.".to_string(),
    ];

    let request = EmbeddingsRequest::new("text-embedding-ada-002".to_string(), inputs)
        .input_type("document".to_string());

    let response = openai.embeddings().create(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

### Moderations API

#### Create a Moderation

**Functionality:**  
Classify content for potential policy violations using OpenAI's moderation models.

**Method:**  
POST `/v1/moderations`

**Parameters:**

- **input (string or Vec<String>, required):**  
  The content to be analyzed.

- **model (Option<String>):**  
  The model to use for moderation. Defaults to the latest available model.

**Response:**

- **id (string):**  
  Unique identifier for the moderation.

- **model (string):**  
  The model used for moderation.

- **results (Vec<ModerationResult>):**  
  Array of moderation results corresponding to each input.

**Usage Example:**

```rust
use serde_json::json;
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::moderation::ModerationRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let inputs = vec![
        "I want to harm others.".to_string(),
        "I love sunny days and hiking.".to_string(),
    ];

    let request = ModerationRequest::new(inputs)
        .model("text-moderation-stable".to_string());

    let response = openai.moderations().create(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

### Files API

#### Upload a File

**Functionality:**  
Upload a file that contains document(s) to be processed by OpenAI's models.

**Method:**  
POST `/v1/files`

**Parameters:**

- **file (PathBuf, required):**  
  The path to the file to be uploaded.

- **purpose (string, required):**  
  The purpose of the file (e.g., `"fine-tune"`).

**Response:**

- **id (string):**  
  Unique identifier for the file.

- **object (string):**  
  The object type, typically `"file"`.

- **bytes (u64):**  
  Size of the file in bytes.

- **created_at (u64):**  
  Timestamp of when the file was created.

- **filename (string):**  
  The name of the uploaded file.

- **purpose (string):**  
  The purpose of the file.

**Usage Example:**

```rust
use std::path::PathBuf;
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::file::FileRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let file_path = PathBuf::from("path/to/your/file.jsonl");
    let purpose = "fine-tune".to_string();

    let request = FileRequest::new(file_path, purpose);

    let response = openai.files().upload(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

#### List Files

**Functionality:**  
Retrieve a list of files that have been uploaded to your organization.

**Method:**  
GET `/v1/files`

**Parameters:**

- **limit (Option<u32>):**  
  The number of files to retrieve.

- **page (Option<u32>):**  
  The page number for pagination.

**Response:**

- **object (string):**  
  The object type, typically `"list"`.

- **data (Vec<File>):**  
  Array of file objects.

**Usage Example:**

```rust
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::file::ListFilesRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let request = ListFilesRequest::new()
        .limit(10)
        .page(1);

    let response = openai.files().list(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

### Fine-Tunes API

#### Create a Fine-Tune

**Functionality:**  
Create a fine-tuned model based on a pre-trained OpenAI model and a dataset.

**Method:**  
POST `/v1/fine-tunes`

**Parameters:**

- **training_file (string, required):**  
  The ID of the file to use for training.

- **model (string, required):**  
  The base model to fine-tune (e.g., `"davinci"`).

- **n_epochs (Option<u32>):**  
  The number of epochs to train for. Default is 4.

- **batch_size (Option<u32>):**  
  The batch size to use for training. Default is determined by the base model.

- **learning_rate_multiplier (Option<f64>):**  
  The learning rate multiplier to use for training. Default is 0.1.

- **prompt_loss_weight (Option<f64>):**  
  The weight to use for the prompt loss. Default is 0.01.

- **compute_classification_metrics (Option<bool>):**  
  Whether to compute classification metrics. Default is false.

- **classification_n_classes (Option<u32>):**  
  The number of classes in a classification task.

- **classification_positive_class (Option<String>):**  
  The positive class in a classification task.

- **classification_betas (Option<Vec<f64>>):**  
  The set of betas to use for computing F-beta scores.

**Response:**

- **id (string):**  
  Unique identifier for the fine-tune.

- **object (string):**  
  The object type, typically `"fine-tune"`.

- **model (string):**  
  The model being fine-tuned.

- **status (string):**  
  The status of the fine-tune (`"pending"`, `"running"`, `"succeeded"`, `"failed"`).

- **created_at (u64):**  
  Timestamp of when the fine-tune was created.

- **updated_at (u64):**  
  Timestamp of the last update.

- **events (Vec<FineTuneEvent>):**  
  List of events related to the fine-tune.

**Usage Example:**

```rust
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::fine_tune::FineTuneRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let training_file_id = "file-abc123";
    let base_model = "davinci".to_string();

    let request = FineTuneRequest::new(training_file_id, base_model)
        .n_epochs(5)
        .batch_size(8)
        .learning_rate_multiplier(0.05)
        .prompt_loss_weight(0.02);

    let response = openai.fine_tunes().create(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

#### List Fine-Tunes

**Functionality:**  
Retrieve a list of all fine-tuning jobs for your organization.

**Method:**  
GET `/v1/fine-tunes`

**Parameters:**

- **limit (Option<u32>):**  
  The number of fine-tunes to retrieve.

- **status (Option<String>):**  
  Filter fine-tunes by status (`"completed"`, `"pending"`, etc.).

**Response:**

- **object (string):**  
  The object type, typically `"list"`.

- **data (Vec<FineTune>):**  
  Array of fine-tune objects.

**Usage Example:**

```rust
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::fine_tune::ListFineTunesRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let request = ListFineTunesRequest::new()
        .limit(5)
        .status("succeeded".to_string());

    let response = openai.fine_tunes().list(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

#### Retrieve a Fine-Tune

**Functionality:**  
Retrieve information about a specific fine-tuning job.

**Method:**  
GET `/v1/fine-tunes/{fine_tune_id}`

**Parameters:**

- **fine_tune_id (string, required):**  
  The ID of the fine-tune to retrieve.

**Response:**

- **id (string):**  
  Unique identifier for the fine-tune.

- **object (string):**  
  The object type, typically `"fine-tune"`.

- **model (string):**  
  The model being fine-tuned.

- **status (string):**  
  The status of the fine-tune.

- **created_at (u64):**  
  Timestamp of creation.

- **updated_at (u64):**  
  Timestamp of the last update.

- **events (Vec<FineTuneEvent>):**  
  List of events related to the fine-tune.

**Usage Example:**

```rust
use rusty_openai::openai::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let fine_tune_id = "ft-123456";

    let response = openai.fine_tunes().retrieve(fine_tune_id).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

### Models API

#### List Models

**Functionality:**  
Retrieve a list of available models.

**Method:**  
GET `/v1/models`

**Parameters:**

- **limit (Option<u32>):**  
  The number of models to retrieve.

**Response:**

- **object (string):**  
  The object type, typically `"list"`.

- **data (Vec<Model>):**  
  Array of model objects.

**Usage Example:**

```rust
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::model::ListModelsRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let request = ListModelsRequest::new().limit(10);

    let response = openai.models().list(request).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

#### Retrieve a Model

**Functionality:**  
Retrieve details about a specific model.

**Method:**  
GET `/v1/models/{model_id}`

**Parameters:**

- **model_id (string, required):**  
  The ID of the model to retrieve.

**Response:**

- **id (string):**  
  Unique identifier for the model.

- **object (string):**  
  The object type, typically `"model"`.

- **created (u64):**  
  Timestamp of creation.

- **owned_by (string):**  
  The owner of the model.

- **permissions (Vec<Permission>):**  
  Permissions associated with the model.

**Usage Example:**

```rust
use rusty_openai::openai::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let model_id = "text-davinci-003";

    let response = openai.models().retrieve(model_id).await?;

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

### Utilties

#### Utility Functions

- **strings_to_json_array(strings: &[String]) -> Value:**  
  Convert an array of strings to a JSON array.

  ```rust
  use serde_json::Value;

  fn strings_to_json_array(strings: &[String]) -> Value {
      serde_json::to_value(strings).unwrap()
  }
  ```

- **insert_optional_param(params: &mut Map<String, Value>, key: &str, value: Option<impl Into<Value>>):**  
  Handle optional parameters for API requests by inserting them into the parameters map if they are `Some`.

  ```rust
  use serde_json::{Map, Value};

  fn insert_optional_param<K, V>(params: &mut Map<String, Value>, key: K, value: Option<V>)
  where
      K: Into<String>,
      V: Into<Value>,
  {
      if let Some(v) = value {
          params.insert(key.into(), v.into());
      }
  }
  ```

---

## Getting Started

Before using the SDK, ensure you have the necessary dependencies added to your `Cargo.toml` file:

```toml
[dependencies]
rusty_openai = "0.1.6"
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12.5", features = ["json", "multipart"] }
```

### Initialize OpenAI Client

Create an instance of OpenAI with your API key:

```rust
use rusty_openai::openai::OpenAI;

#[tokio::main]
async fn main() {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");
}
```

### Example: Generate a Chat Completion

```rust
use serde_json::json;
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::completion::ChatCompletionRequest;

#[tokio::main]
async fn main() {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");

    let messages = vec![
        json!({
            "role": "user",
            "content": "Hello, how are you?"
        }),
    ];

    let request = ChatCompletionRequest::new("gpt-4".to_string(), messages)
        .max_tokens(150)
        .temperature(0.7);

    match openai.completions().create(request).await {
        Ok(response) => println!("{}", serde_json::to_string_pretty(&response).unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}