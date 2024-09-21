use crate::{error_handling::OpenAIResult, extend_url_params, openai::OpenAI, setters};
use serde::Serialize;
use serde_json::{json, Value};

/// [`ThreadsApi`] struct to interact with thread management endpoints of the API.
pub struct ThreadsApi<'a>(pub(crate) &'a OpenAI<'a>);

/// Struct representing a request to create a thread.
#[derive(Default, Serialize)]
pub struct ThreadCreationRequest {
    /// Optional list of messages in the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<Value>>,

    /// Optional tool resources related to the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_resources: Option<Value>,

    /// Optional metadata associated with the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,
}

/// Struct representing a request to modify a thread.
#[derive(Default, Serialize)]
pub struct ThreadModificationRequest {
    /// Optional tool resources related to the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_resources: Option<Value>,

    /// Optional metadata associated with the thread
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,
}

impl ThreadCreationRequest {
    setters! {
        /// Set messages for the thread.
        messages: Vec<Value>,

        /// Set tool resources for the thread.
        tool_resources: Value,

        /// Set metadata for the thread.
        metadata: Value,
    }
}

impl ThreadModificationRequest {
    setters! {
        /// Set tool resources for the thread.
        tool_resources: Value,

        /// Set metadata for the thread.
        metadata: Value,
    }
}

#[derive(Serialize)]
struct CreateMessageRequest<'a> {
    /// The role of the message sender.
    role: &'a str,

    /// The content of the message.
    content: Value,

    /// Optional attachments for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Value>,

    /// Optional metadata for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,
}

#[derive(Serialize)]
struct ModifyMessageRequest {
    /// The new metadata to apply to the message.
    metadata: Value,
}

#[derive(Serialize)]
struct CreateRunRequest<'a> {
    assistant_id: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    additional_instructions: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    additional_messages: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_prompt_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    truncation_strategy: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_tool_calls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<Value>,
}

#[derive(Serialize)]
struct SubmitToolRequest {
    /// List of tool outputs to submit.
    tool_outputs: Vec<Value>,

    /// Optional stream parameter for the submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

impl<'a> ThreadsApi<'a> {
    /// Create a new thread with the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A [`ThreadCreationRequest`] containing the parameters for the new thread.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn create(&self, request: ThreadCreationRequest) -> OpenAIResult<Value> {
        self.0.post_json("/threads", &request).await
    }

    /// Retrieve the details of a specific thread by its ID.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn retrieve(&self, thread_id: &str) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}");

        self.0.get(&url).await
    }

    /// Modify an existing thread's details using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to be modified.
    /// * `request` - A [`ThreadModificationRequest`] containing the modification parameters.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn modify(
        &self,
        thread_id: &str,
        request: ThreadModificationRequest,
    ) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}");

        self.0.post_json(&url, &request).await
    }

    /// Delete a specific thread by its ID.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The ID of the thread to be deleted.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn delete(&self, thread_id: &str) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}");

        self.0.delete(&url).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn create_message(
        &self,
        thread_id: &str,
        role: &str,
        content: Value,
        attachments: Option<Value>,
        metadata: Option<Value>,
    ) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/messages");
        let body = CreateMessageRequest {
            role,
            content,
            attachments,
            metadata,
        };

        self.0.post_json(&url, &body).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn list_messages(
        &self,
        thread_id: &str,
        limit: Option<u32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> OpenAIResult<Value> {
        let mut url = format!("/threads/{thread_id}/messages?");

        extend_url_params!(url, limit, order, after, before);
        url.pop();

        self.0.get(&url).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn retrieve_message(&self, thread_id: &str, message_id: &str) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/messages/{message_id}");

        self.0.get(&url).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn modify_message(
        &self,
        thread_id: &str,
        message_id: &str,
        metadata: Value,
    ) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/messages/{message_id}");
        let body = ModifyMessageRequest { metadata };

        self.0.post_json(&url, &body).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn delete_message(&self, thread_id: &str, message_id: &str) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/messages/{message_id}");

        self.0.delete(&url).await
    }

    /// Create and initiate a run in a specific thread with specified parameters.
    ///
    /// # Arguments
    ///
    /// * Various parameters used to customize the creation of the run.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
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
    ) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/runs");
        let body = CreateRunRequest {
            assistant_id,
            model,
            instructions,
            additional_instructions,
            additional_messages,
            tools,
            metadata,
            temperature,
            top_p,
            stream,
            max_prompt_tokens,
            max_completion_tokens,
            truncation_strategy,
            tool_choice,
            parallel_tool_calls,
            response_format,
        };

        self.0.post_json(&url, &body).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn list_runs(
        &self,
        thread_id: &str,
        limit: Option<u32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> OpenAIResult<Value> {
        let mut url = format!("/threads/{thread_id}/runs?");

        extend_url_params!(url, limit, order, after, before);
        url.pop();

        self.0.get(&url).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn retrieve_run(&self, thread_id: &str, run_id: &str) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/runs/{run_id}");

        self.0.get(&url).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn modify_run(
        &self,
        thread_id: &str,
        run_id: &str,
        metadata: Value,
    ) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/runs/{run_id}");
        let body = ModifyMessageRequest { metadata };

        self.0.post_json(&url, &body).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn delete_run(&self, thread_id: &str, run_id: &str) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/runs/{run_id}");

        self.0.delete(&url).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn submit_tool_outputs(
        &self,
        thread_id: &str,
        run_id: &str,
        tool_outputs: Vec<Value>,
        stream: Option<bool>,
    ) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/runs/{run_id}/submit_tool_outputs");
        let body = SubmitToolRequest {
            tool_outputs,
            stream,
        };

        self.0.post_json(&url, &body).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn cancel_run(&self, thread_id: &str, run_id: &str) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/runs/{run_id}/cancel");
        let body = json!({});

        self.0.post_json(&url, &body).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn list_run_steps(
        &self,
        thread_id: &str,
        run_id: &str,
        limit: Option<u32>,
        order: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> OpenAIResult<Value> {
        let mut url = format!("/threads/{thread_id}/runs/{run_id}/steps?");

        extend_url_params!(url, limit, order, after, before);
        url.pop();

        self.0.get(&url).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn retrieve_run_step(
        &self,
        thread_id: &str,
        run_id: &str,
        step_id: &str,
    ) -> OpenAIResult<Value> {
        let url = format!("/threads/{thread_id}/runs/{run_id}/steps/{step_id}");

        self.0.get(&url).await
    }
}
