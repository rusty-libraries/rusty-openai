# OpenAI Rust SDK

![Crates.io](https://img.shields.io/crates/v/rusty-openai) ![docs.rs](https://img.shields.io/docsrs/rusty-openai) ![License](https://img.shields.io/crates/l/rusty-openai)

Welcome to the OpenAI Rust SDK, your all-in-one solution for integrating OpenAI's powerful capabilities into your Rust projects. This SDK provides a convenient abstraction over OpenAI's API, enabling you to easily perform tasks such as generating completions, creating and editing images, moderating text, fine-tuning models, and more.

## Table of Contents

- [OpenAI Rust SDK](#openai-rust-sdk)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Getting Started](#getting-started)
    - [Initialize OpenAI Client](#initialize-openai-client)
    - [Generate Chat Completions](#generate-chat-completions)
  - [Documentation](#documentation)
  - [License](#license)

## Installation

To use this SDK, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
rusty-openai = "0.1.8"
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12.5", features = ["json", "multipart"] }
```

## Getting Started

To get started with the OpenAI Rust SDK, follow these steps:

### Initialize OpenAI Client

First, create an instance of the `OpenAI` struct with your API key.

```rust
use rusty_openai::openai::OpenAI;

#[tokio::main]
async fn main() {
    let openai = OpenAI::new("YOUR_API_KEY", "https://api.openai.com/v1");
}
```

### Generate Chat Completions

To generate chat completions, create a `ChatCompletionRequest` object and call the `create` method from the completions API:

```rust
use rusty_openai::openai::OpenAI;
use rusty_openai::openai_api::completion::ChatCompletionRequest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("API key not set");
    let openai = OpenAI::new(&api_key, "https://api.openai.com/v1");

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

This simple example demonstrates how to generate chat completions using the SDK. For more detailed usage and additional endpoints, refer to the [documentation](#documentation).

## Documentation

For detailed information on all the available endpoints and their respective methods, please refer to the full [SDK Documentation](https://pleaseful.github.io/rusty-openai/).

## License

This SDK is licensed under the MIT License. For more details, see the [LICENSE](LICENSE.md) file.

## NOTE

Now please do be noted that this library is lacking of DETAILED documentations, as well as other missing endpoints from the official one. You may be asking why am I creating this library on Rust when there's already a repository and a library for it on Rust.

### Why?

The current one does not support images and is lacking of functions and is not actively maintained.

---

Happy coding with OpenAI and Rust! If you encounter any issues or have questions, feel free to open an issue on the GitHub repository. Contributions and improvements are always welcome.

---