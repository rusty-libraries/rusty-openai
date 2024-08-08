use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

/// VectorsApi struct to interact with vector stores API endpoints.
pub struct VectorsApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

/// Struct representing a request for vector store creation and modification.
pub struct VectorStoreRequest {
    file_ids: Option<Vec<String>>,        // List of file IDs to include in the vector store
    name: Option<String>,                 // Name for the vector store
    expires_after: Option<Value>,         // Expiration date for the vector store
    chunking_strategy: Option<Value>,     // Strategy for chunking the data
    metadata: Option<Value>,              // Metadata for the vector store
}

impl VectorStoreRequest {
    /// Create a new instance of VectorStoreRequest.
    pub fn new() -> Self {
        VectorStoreRequest {
            file_ids: None,
            name: None,
            expires_after: None,
            chunking_strategy: None,
            metadata: None,
        }
    }

    /// Set file IDs for the request.
    pub fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
        self
    }

    /// Set name for the request.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set expiration date for the request.
    pub fn expires_after(mut self, expires_after: Value) -> Self {
        self.expires_after = Some(expires_after);
        self
    }

    /// Set chunking strategy for the request.
    pub fn chunking_strategy(mut self, chunking_strategy: Value) -> Self {
        self.chunking_strategy = Some(chunking_strategy);
        self
    }

    /// Set metadata for the request.
    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl<'a> VectorsApi<'a> {
    /// Create a new instance of VectorsApi.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        VectorsApi { client, base_url }
    }

    /// Create a new vector store using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A VectorStoreRequest containing the parameters for the vector store.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn create_vector_store(&self, request: VectorStoreRequest) -> Result<Value, OpenAIError> {
        // Construct the full URL for the vector stores endpoint.
        let url = format!("{}/vector_stores", self.base_url);

        // Initialize a JSON map to build the request body.
        let mut body = serde_json::Map::new();

        // Insert optional fields if they are provided.
        if let Some(file_ids) = request.file_ids {
            body.insert("file_ids".to_string(), json!(file_ids));
        }
        if let Some(name) = request.name {
            body.insert("name".to_string(), json!(name));
        }
        if let Some(expires_after) = request.expires_after {
            body.insert("expires_after".to_string(), json!(expires_after));
        }
        if let Some(chunking_strategy) = request.chunking_strategy {
            body.insert("chunking_strategy".to_string(), json!(chunking_strategy));
        }
        if let Some(metadata) = request.metadata {
            body.insert("metadata".to_string(), json!(metadata));
        }

        // Send a POST request to the vector stores endpoint with the request body.
        let response = self.client.post(&url, &Value::Object(body)).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;

        // Return the parsed JSON response.
        Ok(json)
    }

    /// List vector stores with optional query parameters.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of vector stores to retrieve.
    /// * `order` - Order of the retrieved vector stores.
    /// * `after` - Retrieve vector stores created after this ID.
    /// * `before` - Retrieve vector stores created before this ID.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn list_vector_stores(&self, limit: Option<u64>, order: Option<String>, after: Option<String>, before: Option<String>) -> Result<Value, OpenAIError> {
        let mut url = format!("{}/vector_stores", self.base_url);
        url.push_str("?");
        if let Some(limit) = limit {
            url.push_str(&format!("limit={}&", limit));
        }
        if let Some(order) = order {
            url.push_str(&format!("order={}&", order));
        }
        if let Some(after) = after {
            url.push_str(&format!("after={}&", after));
        }
        if let Some(before) = before {
            url.push_str(&format!("before={}&", before));
        }
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Retrieve details of a specific vector store.
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn retrieve_vector_store(&self, vector_store_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/vector_stores/{}", self.base_url, vector_store_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Modify an existing vector store using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to modify.
    /// * `request` - A VectorStoreRequest containing the parameters for the vector store modification.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn modify_vector_store(&self, vector_store_id: &str, request: VectorStoreRequest) -> Result<Value, OpenAIError> {
        let url = format!("{}/vector_stores/{}", self.base_url, vector_store_id);
        let mut body = serde_json::Map::new();

        if let Some(name) = request.name {
            body.insert("name".to_string(), json!(name));
        }
        if let Some(expires_after) = request.expires_after {
            body.insert("expires_after".to_string(), json!(expires_after));
        }
        if let Some(metadata) = request.metadata {
            body.insert("metadata".to_string(), json!(metadata));
        }

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Delete a specific vector store.
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn delete_vector_store(&self, vector_store_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/vector_stores/{}", self.base_url, vector_store_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
}