use crate::{error_handling::OpenAIResult, openai::OpenAI};
use serde::Serialize;
use serde_json::Value;

/// EmbeddingsApi struct to interact with the embeddings endpoint of the API.
pub struct EmbeddingsApi<'a>(pub(crate) &'a OpenAI);

#[derive(Serialize)]
struct AssistantRequest<'a> {
    /// The input text for which to create embeddings.
    input: &'a str,

    /// Embedding model to use
    model: &'a str,

    /// Optional encoding format
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<&'a str>,

    /// Optional number of dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<u64>,

    /// Optional user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a str>,
}

impl<'a> EmbeddingsApi<'a> {
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
        input: &str,
        model: &str,                   // Embedding model to use
        encoding_format: Option<&str>, // Optional encoding format
        dimensions: Option<u64>,       // Optional number of dimensions
        user: Option<&str>,            // Optional user ID
    ) -> OpenAIResult<Value> {
        // Initialize a JSON object to build the request body.
        let body = AssistantRequest {
            input,
            model,
            encoding_format,
            dimensions,
            user,
        };

        // Send a POST request to the embeddings endpoint with the request body.
        self.0.post_json("/embeddings", &body).await
    }
}
