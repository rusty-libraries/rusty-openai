use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

/// AssistantsApi struct to interact with the assistants endpoints of the API.
pub struct AssistantsApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

/// Struct representing a request for creating or modifying an assistant.
pub struct AssistantRequest {
    model: String,                // Model name to be used for the assistant
    name: Option<String>,         // Name for the assistant
    description: Option<String>,  // Description of the assistant
    instructions: Option<String>, // Instructions for the assistant
    tools: Option<Vec<Value>>,    // Tools to be used by the assistant
    tool_resources: Option<Value>,// Resources for the tools
    metadata: Option<Value>,      // Metadata for the assistant
    temperature: Option<f64>,     // Sampling temperature
    top_p: Option<f64>,           // Nucleus sampling parameter
    response_format: Option<Value>,// Format of responses from the assistant
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
    /// Create a new instance of AssistantsApi.
    ///
    /// # Arguments
    ///
    /// * `client` - Reference to the HTTP client.
    /// * `base_url` - Base URL for the API.
    ///
    /// # Returns
    ///
    /// A new instance of AssistantsApi.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        AssistantsApi { client, base_url }
    }

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
    pub async fn create(&self, request: AssistantRequest) -> Result<Value, OpenAIError> {
        // Construct the full URL for the assistants endpoint.
        let url = format!("{}/assistants", self.base_url);

        // Initialize a JSON map to build the request body.
        let mut body = serde_json::Map::new();
        body.insert("model".to_string(), json!(request.model));

        // Insert optional fields if they are provided.
        if let Some(name) = request.name {
            body.insert("name".to_string(), json!(name));
        }
        if let Some(description) = request.description {
            body.insert("description".to_string(), json!(description));
        }
        if let Some(instructions) = request.instructions {
            body.insert("instructions".to_string(), json!(instructions));
        }
        if let Some(tools) = request.tools {
            body.insert("tools".to_string(), json!(tools));
        }
        if let Some(tool_resources) = request.tool_resources {
            body.insert("tool_resources".to_string(), json!(tool_resources));
        }
        if let Some(metadata) = request.metadata {
            body.insert("metadata".to_string(), json!(metadata));
        }
        if let Some(temperature) = request.temperature {
            body.insert("temperature".to_string(), json!(temperature));
        }
        if let Some(top_p) = request.top_p {
            body.insert("top_p".to_string(), json!(top_p));
        }
        if let Some(response_format) = request.response_format {
            body.insert("response_format".to_string(), json!(response_format));
        }

        // Send a POST request to the assistants endpoint with the request body.
        let response = self.client.post(&url, &Value::Object(body)).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;

        // Return the parsed JSON response.
        Ok(json)
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
    pub async fn list(&self, limit: Option<u32>, order: Option<&str>, after: Option<&str>, before: Option<&str>) -> Result<Value, OpenAIError> {
        let mut url = format!("{}/assistants", self.base_url);
        let mut params = vec![];

        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
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
    pub async fn retrieve(&self, assistant_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/assistants/{}", self.base_url, assistant_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
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
    pub async fn modify(&self, assistant_id: &str, request: AssistantRequest) -> Result<Value, OpenAIError> {
        let url = format!("{}/assistants/{}", self.base_url, assistant_id);
        let mut body = serde_json::Map::new();

        // Insert optional fields if they are provided.
        if let Some(name) = request.name {
            body.insert("name".to_string(), json!(name));
        }
        if let Some(description) = request.description {
            body.insert("description".to_string(), json!(description));
        }
        if let Some(instructions) = request.instructions {
            body.insert("instructions".to_string(), json!(instructions));
        }
        if let Some(tools) = request.tools {
            body.insert("tools".to_string(), json!(tools));
        }
        if let Some(tool_resources) = request.tool_resources {
            body.insert("tool_resources".to_string(), json!(tool_resources));
        }
        if let Some(metadata) = request.metadata {
            body.insert("metadata".to_string(), json!(metadata));
        }
        if let Some(temperature) = request.temperature {
            body.insert("temperature".to_string(), json!(temperature));
        }
        if let Some(top_p) = request.top_p {
            body.insert("top_p".to_string(), json!(top_p));
        }
        if let Some(response_format) = request.response_format {
            body.insert("response_format".to_string(), json!(response_format));
        }

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
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
    pub async fn delete(&self, assistant_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/assistants/{}", self.base_url, assistant_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
}