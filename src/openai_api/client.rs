use crate::{error_handling::OpenAIResult, openai::OpenAI};
use serde_json::Value;

/// ClientApi struct to interact with the models endpoint of the API.
pub struct ClientApi<'a>(pub(crate) &'a OpenAI);

impl<'a> ClientApi<'a> {
    /// Fetch the list of available models from the API.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn get_models(&self) -> OpenAIResult<Value> {
        // Send a GET request to the models endpoint.
        self.0.get("/models").await
    }
}
