use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

/// CompletionsApi struct to interact with the chat completions endpoint of the API.
pub struct CompletionsApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

/// Struct representing a request for chat completions.
pub struct ChatCompletionRequest {
    model: String,                // Model name to be used for the chat completion
    messages: Vec<Value>,         // History of messages in the conversation
    max_tokens: Option<u64>,      // Maximum number of tokens to generate
    temperature: Option<f64>,     // Sampling temperature
    top_p: Option<f64>,           // Nucleus sampling parameter
    n: Option<u64>,               // Number of completions to generate for each prompt
    stream: Option<bool>,         // Whether to stream back partial progress
    stop: Option<Vec<String>>,    // Sequence to stop generation
    presence_penalty: Option<f64>,// Presence penalty to apply
    frequency_penalty: Option<f64>,// Frequency penalty to apply
    logit_bias: Option<Value>,    // Bias for logits
    user: Option<String>,         // User ID
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
    /// Create a new instance of CompletionsApi.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        CompletionsApi { client, base_url }
    }

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
    pub async fn create(&self, request: ChatCompletionRequest) -> Result<Value, OpenAIError> {
        // Construct the full URL for the chat completions endpoint.
        let url = format!("{}/chat/completions", self.base_url);
        
        // Initialize a JSON map to build the request body.
        let mut body = serde_json::Map::new();
    
        // Insert required fields to the JSON map.
        body.insert("model".to_string(), json!(request.model));
        body.insert("messages".to_string(), json!(request.messages));
        
        // Insert optional fields if they are provided.
        if let Some(max_tokens) = request.max_tokens {
            body.insert("max_tokens".to_string(), json!(max_tokens));
        }
        if let Some(temperature) = request.temperature {
            body.insert("temperature".to_string(), json!(temperature));
        }
        if let Some(top_p) = request.top_p {
            body.insert("top_p".to_string(), json!(top_p));
        }
        if let Some(n) = request.n {
            body.insert("n".to_string(), json!(n));
        }
        if let Some(stream) = request.stream {
            body.insert("stream".to_string(), json!(stream));
        }
        if let Some(stop) = request.stop {
            body.insert("stop".to_string(), json!(stop));
        }
        if let Some(presence_penalty) = request.presence_penalty {
            body.insert("presence_penalty".to_string(), json!(presence_penalty));
        }
        if let Some(frequency_penalty) = request.frequency_penalty {
            body.insert("frequency_penalty".to_string(), json!(frequency_penalty));
        }
        if let Some(logit_bias) = request.logit_bias {
            body.insert("logit_bias".to_string(), json!(logit_bias));
        }
        if let Some(user) = request.user {
            body.insert("user".to_string(), json!(user));
        }

        // Send a POST request to the chat completions endpoint with the request body.
        let response = self.client.post(&url, &Value::Object(body)).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}