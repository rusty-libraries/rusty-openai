# OpenAI Rust SDK Documentation

Welcome to the OpenAI Rust SDK documentation. This SDK allows you to interact with OpenAI's API endpoints to perform various tasks such as generating completions, creating images, moderating text, etc.

## Getting Started

Before using the SDK, make sure you have the following dependencies added to your `Cargo.toml` file:

```toml
[dependencies]
rusty_openai = "0.1.0"
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
pub async fn create(&self, request: ChatCompletionRequest) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `request`: `ChatCompletionRequest`
    - `model`: The model name to be used for the chat completion.
    - `messages`: The history of messages in the conversation.
    - Fluent setter methods for additional options like `max_tokens`, `temperature`, etc.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


#### Images API

**Generate Image**

Generate an image based on the provided prompt and parameters:

```rust
pub async fn generate(
    &self,
    prompt: &str,
    model: &str,
    size: Option<&str>,
    response_format: Option<&str>,
    n: Option<u64>,
    user: Option<&str>
) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `prompt`: The text prompt to generate the image from.
  - `model`: The name of the model to use for generating the image.
  - `size`: Optional size of the image.
  - `response_format`: Optional response format (e.g., `json`, `url`).
  - `n`: Optional number of images to generate.
  - `user`: Optional user ID.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


**Edit Image**

Edit an existing image using the provided parameters and mask:

```rust
pub async fn edit(
    &self,
    model: &str,
    image_path: &str,
    mask_path: &str,
    prompt: &str,
    size: Option<&str>,
    response_format: Option<&str>,
    n: Option<u64>,
    user: Option<&str>
) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `model`: The model to use for editing the image.
  - `image_path`: Local file path to the image.
  - `mask_path`: Local file path to the mask.
  - `prompt`: The text prompt to guide the editing.
  - `size`: Optional size of the edited image.
  - `response_format`: Optional response format (e.g., `json`, `url`).
  - `n`: Optional number of edited images to generate.
  - `user`: Optional user ID.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


**Create Image Variation**

Create variations of an existing image using the provided parameters:

```rust
pub async fn variation(
    &self,
    model: &str,
    image_path: &str,
    size: Option<&str>,
    response_format: Option<&str>,
    n: Option<u64>,
    user: Option<&str>
) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `model`: The model to use for generating variations.
  - `image_path`: Local file path to the image.
  - `size`: Optional size of the variation images.
  - `response_format`: Optional response format (e.g., `json`, `url`).
  - `n`: Optional number of variation images to generate.
  - `user`: Optional user ID.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


#### Moderation API

**Submit Text for Moderation**

Submit text input for moderation:

```rust
pub async fn moderate(
    &self,
    input: &str,
    model: Option<&str>
) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `input`: The text input to be moderated.
  - `model`: Optional name of the moderation model to use.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


#### Fine-Tuning API

**Create Fine-Tuning Job**

Create a new fine-tuning job with the specified parameters:

```rust
pub async fn create_fine_tuning_job(
    &self,
    model: &str,
    training_file: &str,
    validation_file: Option<&str>,
    n_epochs: Option<u32>,
    batch_size: Option<u32>,
    learning_rate_multiplier: Option<f64>,
    prompt_loss_weight: Option<f64>,
    compute_classification_metrics: Option<bool>,
    classification_n_classes: Option<u32>,
    classification_positive_class: Option<&str>,
    classification_betas: Option<Vec<f64>>
) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `model`: The model to be fine-tuned.
  - `training_file`: The file containing training data.
  - `validation_file`: Optional validation data file.
  - `n_epochs`: Optional number of training epochs.
  - `batch_size`: Optional batch size for training.
  - `learning_rate_multiplier`: Optional learning rate multiplier.
  - `prompt_loss_weight`: Optional weight for the prompt loss.
  - `compute_classification_metrics`: Optional flag to compute classification metrics.
  - `classification_n_classes`: Optional number of classes for classification.
  - `classification_positive_class`: Optional positive class for classification.
  - `classification_betas`: Optional betas for classification metrics.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


**List Fine-Tuning Jobs**

List all fine-tuning jobs:

```rust
pub async fn list_fine_tuning_jobs(&self) -> Result<Value, OpenAIError>
```

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


**Retrieve Fine-Tuning Job**

Retrieve information about a specific fine-tuning job:

```rust
pub async fn retrieve_fine_tuning_job(&self, job_id: &str) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `job_id`: The ID of the fine-tuning job to retrieve.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.


#### Embeddings API

**Create Embedding**

Create an embedding using the provided parameters:

```rust
pub async fn create(
    &self,
    input: &str,
    model: &str,
    encoding_format: Option<&str>,
    dimensions: Option<u64>,
    user: Option<&str>
) -> Result<Value, OpenAIError>
```

- **Parameters:**
  - `input`: The input text for which to create embeddings.
  - `model`: The name of the model to use for creating embeddings.
  - `encoding_format`: Optional encoding format.
  - `dimensions`: Optional number of dimensions for the embeddings.
  - `user`: Optional user ID.

- **Returns:**
  - `Result<Value, OpenAIError>`: A result containing the JSON response as `serde_json::Value` on success, or an `OpenAIError` on failure.