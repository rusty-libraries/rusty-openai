use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::json;
use serde_json::Value;

/// EmbeddingsApi struct to interact with the embeddings endpoint of the API.
pub struct EmbeddingsApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

impl<'a> EmbeddingsApi<'a> {
    /// Create a new instance of EmbeddingsApi.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the RequestClient.
    /// * `base_url` - The base URL for the API.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        EmbeddingsApi { client, base_url }
    }

    /// Create an embedding using the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `input` - The input text for which to create embeddings.
    /// * `model` - The name of the model to use for creating embeddings.
    /// * `encoding_format` - Optional encoding format.
    /// * `dimensions` - Optional number of dimensions for the embeddings.
    /// * `user` - Optional user ID.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn create(
        &self, 
        input: &str,               // Input text for embedding
        model: &str,               // Embedding model to use
        encoding_format: Option<&str>,  // Optional encoding format
        dimensions: Option<u64>,   // Optional number of dimensions
        user: Option<&str>         // Optional user ID
    ) -> Result<Value, OpenAIError> {
        // Construct the full URL for the embeddings endpoint.
        let url = format!("{}/embeddings", self.base_url);
        
        // Initialize a JSON object to build the request body.
        let mut body = json!({
            "input": input,
            "model": model
        });

        // Insert optional fields if they are provided.
        if let Some(encoding_format) = encoding_format {
            if let Value::Object(map) = &mut body {
                map.insert("encoding_format".to_string(), json!(encoding_format));
            }
        }

        if let Some(dimensions) = dimensions {
            if let Value::Object(map) = &mut body {
                map.insert("dimensions".to_string(), json!(dimensions));
            }
        }

        if let Some(user) = user {
            if let Value::Object(map) = &mut body {
                map.insert("user".to_string(), json!(user));
            }
        }

        // Send a POST request to the embeddings endpoint with the request body.
        let response = self.client.post(&url, &body).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}