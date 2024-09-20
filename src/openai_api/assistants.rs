use crate::{error_handling::OpenAIResult, extend_url_params, openai::OpenAI};
use serde::Serialize;
use serde_json::Value;

/// AssistantsApi struct to interact with the assistants endpoints of the API.
pub struct AssistantsApi<'a>(pub(crate) &'a OpenAI);

/// Struct representing a request for creating or modifying an assistant.
#[derive(Serialize)]
pub struct AssistantRequest {
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

impl AssistantRequest {
    /// Create a new instance of AssistantRequest.
    ///
    /// # Arguments
    ///
    /// * `model` - Model name to be used for the assistant.
    ///
    /// # Returns
    ///
    /// A new instance of AssistantRequest.
    pub fn new(model: String) -> Self {
        AssistantRequest {
            model,
            name: None,
            description: None,
            instructions: None,
            tools: None,
            tool_resources: None,
            metadata: None,
            temperature: None,
            top_p: None,
            response_format: None,
        }
    }

    /// Set the name for the assistant request.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the description for the assistant request.
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the instructions for the assistant request.
    pub fn instructions(mut self, instructions: String) -> Self {
        self.instructions = Some(instructions);
        self
    }

    /// Set the tools for the assistant request.
    pub fn tools(mut self, tools: Vec<Value>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set the tool resources for the assistant request.
    pub fn tool_resources(mut self, tool_resources: Value) -> Self {
        self.tool_resources = Some(tool_resources);
        self
    }

    /// Set the metadata for the assistant request.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set the temperature for the assistant request.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the top_p parameter for the assistant request.
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the response format for the assistant request.
    pub fn response_format(mut self, response_format: Value) -> Self {
        self.response_format = Some(response_format);
        self
    }
}

impl<'a> AssistantsApi<'a> {
    /// Create a new assistant using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - An AssistantRequest containing the parameters for the assistant.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn create(&self, request: AssistantRequest) -> OpenAIResult<Value> {
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
    /// * `request` - An AssistantRequest containing the parameters for the assistant modification.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn modify(
        &self,
        assistant_id: &str,
        request: AssistantRequest,
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
