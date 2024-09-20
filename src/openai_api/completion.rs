use crate::{error_handling::OpenAIResult, openai::OpenAI};
use serde::Serialize;
use serde_json::Value;

/// CompletionsApi struct to interact with the chat completions endpoint of the API.
pub struct CompletionsApi<'a>(pub(crate) &'a OpenAI);

/// Struct representing a request for chat completions.
#[derive(Serialize)]
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
    /// Create a new instance of ChatCompletionRequest.
    pub fn new(model: String, messages: Vec<Value>) -> Self {
        ChatCompletionRequest {
            model,
            messages,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }

    // Fluent setter methods to set each option on the request.

    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn n(mut self, n: u64) -> Self {
        self.n = Some(n);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn logit_bias(mut self, logit_bias: Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

impl<'a> CompletionsApi<'a> {
    /// Create a chat completion using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A ChatCompletionRequest containing the parameters for the completion.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn create(&self, request: ChatCompletionRequest) -> OpenAIResult<Value> {
        // Send a POST request to the chat completions endpoint with the request body.
        self.0.post_json("/chat/completions", &request).await
    }
}
