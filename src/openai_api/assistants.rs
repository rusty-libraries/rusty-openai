use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

pub struct AssistantsApi<'a> {
    client: &'a RequestClient,
    base_url: &'a str,
}

pub struct AssistantRequest {
    model: String,
    name: Option<String>,
    description: Option<String>,
    instructions: Option<String>,
    tools: Option<Vec<Value>>,
    tool_resources: Option<Value>,
    metadata: Option<Value>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    response_format: Option<Value>,
}

impl AssistantRequest {
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

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn instructions(mut self, instructions: String) -> Self {
        self.instructions = Some(instructions);
        self
    }

    pub fn tools(mut self, tools: Vec<Value>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn tool_resources(mut self, tool_resources: Value) -> Self {
        self.tool_resources = Some(tool_resources);
        self
    }

    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
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

    pub fn response_format(mut self, response_format: Value) -> Self {
        self.response_format = Some(response_format);
        self
    }
}

impl<'a> AssistantsApi<'a> {
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        AssistantsApi { client, base_url }
    }

    pub async fn create(&self, request: AssistantRequest) -> Result<Value, OpenAIError> {
        let url = format!("{}/assistants", self.base_url);
        let mut body = serde_json::Map::new();
        body.insert("model".to_string(), json!(request.model));
        
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

    pub async fn retrieve(&self, assistant_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/assistants/{}", self.base_url, assistant_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn modify(&self, assistant_id: &str, request: AssistantRequest) -> Result<Value, OpenAIError> {
        let url = format!("{}/assistants/{}", self.base_url, assistant_id);
        let mut body = serde_json::Map::new();
        
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

    pub async fn delete(&self, assistant_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/assistants/{}", self.base_url, assistant_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
}