use crate::{error_handling::OpenAIResult, extend_url_params, openai::OpenAI, setters};
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
    setters! {
        /// Set file IDs for the request.
        file_ids: Vec<String>,

        /// Set name for the request.
        name: String,

        /// Set expiration date for the request.
        expires_after: Value,

        /// Set chunking strategy for the request.
        chunking_strategy: Value,

        /// Set metadata for the request.
        metadata: Value,
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
        // Send a POST request to the vector stores endpoint with the request body.
        self.0.post_json("/vector_stores", &request).await
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
        let mut url = String::from("/vector_stores?");

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
        let url = format!("/vector_stores/{vector_store_id}");

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
        let url = format!("/vector_stores/{vector_store_id}");

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
        let url = format!("/vector_stores/{vector_store_id}");

        self.0.delete(&url).await
    }
}
