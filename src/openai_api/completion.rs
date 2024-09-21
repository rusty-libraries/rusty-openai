use crate::{error_handling::OpenAIResult, openai::OpenAI, setters};
use serde::Serialize;
use serde_json::Value;

/// [`CompletionsApi`] struct to interact with the chat completions endpoint of the API.
pub struct CompletionsApi<'a>(pub(crate) &'a OpenAI);

/// Struct representing a request for chat completions.
#[derive(Default, Serialize)]
pub struct ChatCompletionRequest {
    /// Model name to be used for the chat completion
    model: String,

    /// History of messages in the conversation
    messages: Vec<Value>,

    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,

    /// Sampling temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,

    /// Number of completions to generate for each prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u64>,

    /// Whether to stream back partial progress
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,

    /// Sequence to stop generation
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,

    /// Presence penalty to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,

    /// Frequency penalty to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,

    /// Bias for logits
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<Value>,

    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl ChatCompletionRequest {
    /// Create a new instance of [`ChatCompletionRequest`].
    #[inline(always)]
    pub fn new(model: String, messages: Vec<Value>) -> Self {
        Self {
            model,
            messages,
            ..Default::default()
        }
    }

    // Fluent setter methods to set each option on the request.

    setters! {
        max_tokens: u64,
        temperature: f64,
        top_p: f64,
        n: u64,
        stream: bool,
        stop: Vec<String>,
        presence_penalty: f64,
        frequency_penalty: f64,
        logit_bias: Value,
        user: String,
    }
}

impl<'a> CompletionsApi<'a> {
    /// Create a chat completion using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A [`ChatCompletionRequest`] containing the parameters for the completion.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn create(&self, request: ChatCompletionRequest) -> OpenAIResult<Value> {
        // Send a POST request to the chat completions endpoint with the request body.
        self.0.post_json("/chat/completions", &request).await
    }
}
