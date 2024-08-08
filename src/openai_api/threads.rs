use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

pub struct ThreadsApi<'a> {
    client: &'a RequestClient,
    base_url: &'a str,
}

pub struct ThreadRequest {
    messages: Option<Vec<Value>>,
    tool_resources: Option<Value>,
    metadata: Option<Value>,
}

impl ThreadRequest {
    pub fn new() -> Self {
        ThreadRequest {
            messages: None,
            tool_resources: None,
            metadata: None,
        }
    }

    pub fn messages(mut self, messages: Vec<Value>) -> Self {
        self.messages = Some(messages);
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
}

impl<'a> ThreadsApi<'a> {
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        ThreadsApi { client, base_url }
    }

    pub async fn create(&self, request: ThreadRequest) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads", self.base_url);
        let mut body = serde_json::Map::new();

        if let Some(messages) = request.messages {
            body.insert("messages".to_string(), json!(messages));
        }
        if let Some(tool_resources) = request.tool_resources {
            body.insert("tool_resources".to_string(), json!(tool_resources));
        }
        if let Some(metadata) = request.metadata {
            body.insert("metadata".to_string(), json!(metadata));
        }

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn retrieve(&self, thread_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}", self.base_url, thread_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn modify(&self, thread_id: &str, request: ThreadRequest) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}", self.base_url, thread_id);
        let mut body = serde_json::Map::new();

        if let Some(tool_resources) = request.tool_resources {
            body.insert("tool_resources".to_string(), json!(tool_resources));
        }
        if let Some(metadata) = request.metadata {
            body.insert("metadata".to_string(), json!(metadata));
        }

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn delete(&self, thread_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}", self.base_url, thread_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn create_message(
        &self,
        thread_id: &str,
        role: &str,
        content: Value,
        attachments: Option<Value>,
        metadata: Option<Value>,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/messages", self.base_url, thread_id);
        let mut body = serde_json::Map::new();

        body.insert("role".to_string(), json!(role));
        body.insert("content".to_string(), content);

        if let Some(attachments) = attachments {
            body.insert("attachments".to_string(), attachments);
        }

        if let Some(metadata) = metadata {
            body.insert("metadata".to_string(), metadata);
        }

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn list_messages(
        &self,
        thread_id: &str,
        limit: Option<u32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<Value, OpenAIError> {
        let mut url = format!("{}/threads/{}/messages", self.base_url, thread_id);
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

    pub async fn retrieve_message(
        &self,
        thread_id: &str,
        message_id: &str,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/messages/{}", self.base_url, thread_id, message_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn modify_message(
        &self,
        thread_id: &str,
        message_id: &str,
        metadata: Value
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/messages/{}", self.base_url, thread_id, message_id);
        let mut body = serde_json::Map::new();

        body.insert("metadata".to_string(), metadata);

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn delete_message(
        &self,
        thread_id: &str,
        message_id: &str,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/messages/{}", self.base_url, thread_id, message_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn create_run(
        &self,
        thread_id: &str,
        assistant_id: &str,
        model: Option<&str>,
        instructions: Option<&str>,
        additional_instructions: Option<&str>,
        additional_messages: Option<Vec<Value>>,
        tools: Option<Vec<Value>>,
        metadata: Option<Value>,
        temperature: Option<f64>,
        top_p: Option<f64>,
        stream: Option<bool>,
        max_prompt_tokens: Option<u32>,
        max_completion_tokens: Option<u32>,
        truncation_strategy: Option<Value>,
        tool_choice: Option<Value>,
        parallel_tool_calls: Option<bool>,
        response_format: Option<Value>,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/runs", self.base_url, thread_id);
        let mut body = serde_json::Map::new();

        body.insert("assistant_id".to_string(), json!(assistant_id));

        if let Some(model) = model {
            body.insert("model".to_string(), json!(model));
        }
        if let Some(instructions) = instructions {
            body.insert("instructions".to_string(), json!(instructions));
        }
        if let Some(additional_instructions) = additional_instructions {
            body.insert("additional_instructions".to_string(), json!(additional_instructions));
        }
        if let Some(additional_messages) = additional_messages {
            body.insert("additional_messages".to_string(), json!(additional_messages));
        }
        if let Some(tools) = tools {
            body.insert("tools".to_string(), json!(tools));
        }
        if let Some(metadata) = metadata {
            body.insert("metadata".to_string(), json!(metadata));
        }
        if let Some(temperature) = temperature {
            body.insert("temperature".to_string(), json!(temperature));
        }
        if let Some(top_p) = top_p {
            body.insert("top_p".to_string(), json!(top_p));
        }
        if let Some(stream) = stream {
            body.insert("stream".to_string(), json!(stream));
        }
        if let Some(max_prompt_tokens) = max_prompt_tokens {
            body.insert("max_prompt_tokens".to_string(), json!(max_prompt_tokens));
        }
        if let Some(max_completion_tokens) = max_completion_tokens {
            body.insert("max_completion_tokens".to_string(), json!(max_completion_tokens));
        }
        if let Some(truncation_strategy) = truncation_strategy {
            body.insert("truncation_strategy".to_string(), json!(truncation_strategy));
        }
        if let Some(tool_choice) = tool_choice {
            body.insert("tool_choice".to_string(), json!(tool_choice));
        }
        if let Some(parallel_tool_calls) = parallel_tool_calls {
            body.insert("parallel_tool_calls".to_string(), json!(parallel_tool_calls));
        }
        if let Some(response_format) = response_format {
            body.insert("response_format".to_string(), json!(response_format));
        }

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn list_runs(
        &self,
        thread_id: &str,
        limit: Option<u32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<Value, OpenAIError> {
        let mut url = format!("{}/threads/{}/runs", self.base_url, thread_id);
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

    pub async fn retrieve_run(
        &self,
        thread_id: &str,
        run_id: &str,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/runs/{}", self.base_url, thread_id, run_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn modify_run(
        &self,
        thread_id: &str,
        run_id: &str,
        metadata: Value
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/runs/{}", self.base_url, thread_id, run_id);
        let mut body = serde_json::Map::new();

        body.insert("metadata".to_string(), metadata);

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn delete_run(
        &self,
        thread_id: &str,
        run_id: &str,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/runs/{}", self.base_url, thread_id, run_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn submit_tool_outputs(
        &self,
        thread_id: &str,
        run_id: &str,
        tool_outputs: Vec<Value>,
        stream: Option<bool>,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/runs/{}/submit_tool_outputs", self.base_url, thread_id, run_id);
        let mut body = serde_json::Map::new();

        body.insert("tool_outputs".to_string(), json!(tool_outputs));

        if let Some(stream) = stream {
            body.insert("stream".to_string(), json!(stream));
        }

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn cancel_run(
        &self,
        thread_id: &str,
        run_id: &str,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/runs/{}/cancel", self.base_url, thread_id, run_id);
        let response = self.client.post(&url, &Value::Object(serde_json::Map::new())).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
    
    pub async fn list_run_steps(
        &self,
        thread_id: &str,
        run_id: &str,
        limit: Option<u32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> Result<Value, OpenAIError> {
        let mut url = format!("{}/threads/{}/runs/{}/steps", self.base_url, thread_id, run_id);
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

    pub async fn retrieve_run_step(
        &self,
        thread_id: &str,
        run_id: &str,
        step_id: &str,
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}/runs/{}/steps/{}", self.base_url, thread_id, run_id, step_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
}