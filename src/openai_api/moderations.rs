use crate::{error_handling::OpenAIResult, openai::OpenAI};
use serde::Serialize;
use serde_json::Value;

/// [`ModerationApi`] struct to interact with the moderation endpoint of the API.
pub struct ModerationApi<'a>(pub(crate) &'a OpenAI<'a>);

#[derive(Serialize)]
struct ModerationRequest<'a> {
    /// The text input to be moderated
    input: &'a str,

    /// Optional name of the moderation model
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<&'a str>,
}

impl<'a> ModerationApi<'a> {
    /// Submit text input for moderation.
    ///
    /// # Arguments
    ///
    /// * `input` - The text input to be moderated.
    /// * `model` - Optional name of the moderation model to use.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn moderate(&self, input: &str, model: Option<&str>) -> OpenAIResult<Value> {
        // Initialize a JSON object to build the request body.
        let body = ModerationRequest { input, model };

        // Send a POST request to the moderation endpoint with the request body.
        self.0.post_json("/moderations", &body).await
    }
}
