use crate::{error_handling::OpenAIResult, extend_url_params, openai::OpenAI};
use serde::Serialize;
use serde_json::Value;

/// VectorsApi struct to interact with vector stores API endpoints.
pub struct VectorsApi<'a>(pub(crate) &'a OpenAI);

/// Struct representing a request for vector store creation.
#[derive(Default, Serialize)]
pub struct VectorStoreCreationRequest {
    /// List of file IDs to include in the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    file_ids: Option<Vec<String>>,

    /// Name for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Expiration date for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_after: Option<Value>,

    /// Strategy for chunking the data
    #[serde(skip_serializing_if = "Option::is_none")]
    chunking_strategy: Option<Value>,

    /// Metadata for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,
}

/// Struct representing a request for vector store modification.
#[derive(Default, Serialize)]
pub struct VectorStoreModificationRequest {
    /// Name for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Expiration date for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_after: Option<Value>,

    /// Metadata for the vector store
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,
}

impl VectorStoreCreationRequest {
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
    /// Create a new vector store using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `request` - A VectorStoreCreationRequest containing the parameters for the vector store.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn create_vector_store(
        &self,
        request: VectorStoreCreationRequest,
    ) -> OpenAIResult<Value> {
        // Construct the full URL for the vector stores endpoint.
        let url = format!("{}/vector_stores", self.0.base_url);

        // Send a POST request to the vector stores endpoint with the request body.
        self.0.post_json(&url, &request).await
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
    pub async fn list_vector_stores(
        &self,
        limit: Option<u64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> OpenAIResult<Value> {
        let mut url = format!("{}/vector_stores?", self.0.base_url);

        extend_url_params!(url, limit, order, after, before);
        url.pop();

        self.0.get(&url).await
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
    pub async fn retrieve_vector_store(&self, vector_store_id: &str) -> OpenAIResult<Value> {
        let url = format!("{}/vector_stores/{vector_store_id}", self.0.base_url);

        self.0.get(&url).await
    }

    /// Modify an existing vector store using the provided request parameters.
    ///
    /// # Arguments
    ///
    /// * `vector_store_id` - The ID of the vector store to modify.
    /// * `request` - A VectorStoreModificationRequest containing the parameters for the vector store modification.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn modify_vector_store(
        &self,
        vector_store_id: &str,
        request: VectorStoreModificationRequest,
    ) -> OpenAIResult<Value> {
        let url = format!("{}/vector_stores/{vector_store_id}", self.0.base_url);

        self.0.post_json(&url, &request).await
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
    pub async fn delete_vector_store(&self, vector_store_id: &str) -> OpenAIResult<Value> {
        let url = format!("{}/vector_stores/{vector_store_id}", self.0.base_url);

        self.0.delete(&url).await
    }
}
