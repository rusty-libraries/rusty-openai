use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::Value;

/// ClientApi struct to interact with the models endpoint of the API.
pub struct ClientApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

impl<'a> ClientApi<'a> {
    /// Create a new instance of ClientApi.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        ClientApi { client, base_url }
    }

    /// Fetch the list of available models from the API.
    ///
    /// # Returns
    /// 
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn get_models(&self) -> Result<Value, OpenAIError> {
        // Construct the full URL for the models endpoint.
        let url = format!("{}/models", self.base_url);

        // Send a GET request to the models endpoint.
        let response = self.client.get(&url).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}