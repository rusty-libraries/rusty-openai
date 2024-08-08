use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

/// ThreadsApi struct to interact with thread management endpoints of the API.
pub struct ThreadsApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

/// Struct representing a request to create or modify a thread.
pub struct ThreadRequest {
    messages: Option<Vec<Value>>,        // Optional list of messages in the thread
    tool_resources: Option<Value>,       // Optional tool resources related to the thread
    metadata: Option<Value>,             // Optional metadata associated with the thread
}

impl ThreadRequest {
    /// Create a new instance of ThreadRequest with default empty fields.
    pub fn new() -> Self {
        ThreadRequest {
            messages: None,
            tool_resources: None,
            metadata: None,
        }
    }

    /// Set messages for the thread.
    pub fn messages(mut self, messages: Vec<Value>) -> Self {
        self.messages = Some(messages);
        self
    }

    /// Set tool resources for the thread.
    pub fn tool_resources(mut self, tool_resources: Value) -> Self {
        self.tool_resources = Some(tool_resources);
        self
    }

    /// Set metadata for the thread.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl<'a> ThreadsApi<'a> {
    /// Create a new instance of ThreadsApi.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        ThreadsApi { client, base_url }
    }

    /// Create a new thread with the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A ThreadRequest containing the parameters for the new thread.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Retrieve the details of a specific thread by its ID.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn retrieve(&self, thread_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}", self.base_url, thread_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Modify an existing thread's details using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to be modified.
    /// * `request` - A ThreadRequest containing the modification parameters.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Delete a specific thread by its ID.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to be deleted.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn delete(&self, thread_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/threads/{}", self.base_url, thread_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Create a new message in a specific thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to add a message to.
    /// * `role` - The role of the message sender.
    /// * `content` - The content of the message.
    /// * `attachments` - Optional attachments for the message.
    /// * `metadata` - Optional metadata for the message.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// List messages in a specific thread with optional filters.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to list messages from.
    /// * `limit` - Optional limit on the number of messages to list.
    /// * `order` - Optional order parameter for the message listing.
    /// * `after` - Optional parameter to list messages after a specific time.
    /// * `before` - Optional parameter to list messages before a specific time.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Retrieve a specific message by its ID from a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to retrieve the message from.
    /// * `message_id` - The ID of the message to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Modify a specific message's metadata in a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the message.
    /// * `message_id` - The ID of the message to modify.
    /// * `metadata` - The new metadata to apply to the message.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Delete a specific message by its ID in a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the message.
    /// * `message_id` - The ID of the message to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Create and initiate a run in a specific thread with specified parameters.
    ///
    /// # Arguments
    ///
    /// * Various parameters used to customize the creation of the run.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// List runs within a specific thread with optional filters.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to list runs from.
    /// * `limit` - Optional limit on the number of runs to list.
    /// * `order` - Optional order parameter for the run listing.
    /// * `after` - Optional parameter to list runs after a specific time.
    /// * `before` - Optional parameter to list runs before a specific time.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Retrieve details of a specific run by its ID.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the run.
    /// * `run_id` - The ID of the run to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Modify a specific run's metadata in a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the run.
    /// * `run_id` - The ID of the run to modify.
    /// * `metadata` - The new metadata to apply to the run.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Delete a specific run by its ID in a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the run.
    /// * `run_id` - The ID of the run to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Submit tool outputs for a specific run.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the run.
    /// * `run_id` - The ID of the run to submit outputs to.
    /// * `tool_outputs` - List of tool outputs to submit.
    /// * `stream` - Optional stream parameter for the submission.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Cancel a specific run by its ID in a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the run.
    /// * `run_id` - The ID of the run to cancel.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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
    
    /// List steps for a specific run within a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the run.
    /// * `run_id` - The ID of the run to list steps from.
    /// * `limit` - Optional limit on the number of steps to list.
    /// * `order` - Optional order parameter for the steps listing.
    /// * `after` - Optional parameter to list steps after a specific time.
    /// * `before` - Optional parameter to list steps before a specific time.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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

    /// Retrieve a specific step by its ID from a run within a thread.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread containing the run.
    /// * `run_id` - The ID of the run containing the step.
    /// * `step_id` - The ID of the step to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
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