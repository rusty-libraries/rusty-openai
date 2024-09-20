use crate::{error_handling::OpenAIResult, extend_url_params, openai::OpenAI, setters};
use serde::Serialize;
use serde_json::Value;

/// AssistantsApi struct to interact with the assistants endpoints of the API.
pub struct AssistantsApi<'a>(pub(crate) &'a OpenAI);

/// Struct representing a request for creating or modifying an assistant.
#[derive(Default, Serialize)]
pub struct AssistantCreationRequest {
    /// Model name to be used for the assistant
    model: String,

    /// Name for the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Description of the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// Instructions for the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,

    /// Tools to be used by the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Value>>,

    /// Resources for the tools
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_resources: Option<Value>,

    /// Metadata for the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,

    /// Sampling temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,

    /// Format of responses from the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<Value>,
}

/// Struct representing a request for creating or modifying an assistant.
#[derive(Default, Serialize)]
pub struct AssistantModificationRequest {
    /// Name for the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Description of the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// Instructions for the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,

    /// Tools to be used by the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Value>>,

    /// Resources for the tools
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_resources: Option<Value>,

    /// Metadata for the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,

    /// Sampling temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,

    /// Format of responses from the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<Value>,
}

macro_rules! assistant_creation_impl {
    () => {
        setters! {
            /// Set the name for the assistant request.
            name: String,

            /// Set the description for the assistant request.
            description: String,

            /// Set the instructions for the assistant request.
            instructions: String,

            /// Set the tools for the assistant request.
            tools: Vec<Value>,

            /// Set the tool resources for the assistant request.
            tool_resources: Value,

            /// Set the metadata for the assistant request.
            metadata: Value,

            /// Set the temperature for the assistant request.
            temperature: f64,

            /// Set the top_p parameter for the assistant request.
            top_p: f64,

            /// Set the response format for the assistant request.
            response_format: Value,
        }
    };
}

impl AssistantCreationRequest {
    /// Create a new instance of AssistantCreationRequest.
    ///
    /// # Arguments
    ///
    /// * `model` - Model name to be used for the assistant.
    ///
    /// # Returns
    ///
    /// A new instance of AssistantCreationRequest.
    pub fn new(model: String) -> Self {
        Self {
            model,
            ..Default::default()
        }
    }

    assistant_creation_impl!();
}

impl AssistantModificationRequest {
    assistant_creation_impl!();
}

impl<'a> AssistantsApi<'a> {
    /// Create a new assistant using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - An AssistantCreationRequest containing the parameters for the assistant.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn create(&self, request: AssistantCreationRequest) -> OpenAIResult<Value> {
        // Send a POST request to the assistants endpoint with the request body.
        self.0.post_json("/assistants", &request).await
    }

    /// List assistants with optional query parameters.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of assistants to retrieve.
    /// * `order` - Order of the retrieved assistants.
    /// * `after` - Retrieve assistants created after this ID.
    /// * `before` - Retrieve assistants created before this ID.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn list(
        &self,
        limit: Option<u32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> OpenAIResult<Value> {
        let mut url = String::from("/assistants?");

        extend_url_params!(url, limit, order, after, before);
        url.pop();

        self.0.get(&url).await
    }

    /// Retrieve details of a specific assistant.
    ///
    /// # Arguments
    ///
    /// * `assistant_id` - The ID of the assistant to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn retrieve(&self, assistant_id: &str) -> OpenAIResult<Value> {
        let url = format!("/assistants/{assistant_id}");

        self.0.get(&url).await
    }

    /// Modify an existing assistant using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `assistant_id` - The ID of the assistant to modify.
    /// * `request` - An AssistantModificationRequest containing the parameters for the assistant modification.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn modify(
        &self,
        assistant_id: &str,
        request: AssistantModificationRequest,
    ) -> OpenAIResult<Value> {
        let url = format!("/assistants/{assistant_id}");

        self.0.post_json(&url, &request).await
    }

    /// Delete a specific assistant.
    ///
    /// # Arguments
    ///
    /// * `assistant_id` - The ID of the assistant to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn delete(&self, assistant_id: &str) -> OpenAIResult<Value> {
        let url = format!("/assistants/{assistant_id}");

        self.0.delete(&url).await
    }
}
