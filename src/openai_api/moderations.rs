use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

/// ModerationApi struct to interact with the moderation endpoint of the API.
pub struct ModerationApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

impl<'a> ModerationApi<'a> {
    /// Create a new instance of ModerationApi.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the RequestClient.
    /// * `base_url` - The base URL for the API.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        ModerationApi { client, base_url }
    }

    /// Submit text input for moderation.
    ///
    /// # Arguments
    ///
    /// * `input` - The text input to be moderated.
    /// * `model` - Optional name of the moderation model to use.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn moderate(
        &self, 
        input: &str,               // The text input to be moderated
        model: Option<&str>        // Optional name of the moderation model 
    ) -> Result<Value, OpenAIError> {
        // Construct the full URL for the moderation endpoint.
        let url = format!("{}/moderations", self.base_url);
        
        // Initialize a JSON object to build the request body.
        let mut body = json!({
            "input": input
        });

        // Insert the optional model field if provided.
        if let Some(model) = model {
            if let Value::Object(map) = &mut body {
                map.insert("model".to_string(), json!(model));
            }
        }

        // Send a POST request to the moderation endpoint with the request body.
        let response = self.client.post(&url, &body).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}