use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

pub struct VectorsApi<'a> {
    client: &'a RequestClient,
    base_url: &'a str,
}

pub struct VectorStoreRequest {
    file_ids: Option<Vec<String>>,
    name: Option<String>,
    expires_after: Option<Value>,
    chunking_strategy: Option<Value>,
    metadata: Option<Value>,
}

impl VectorStoreRequest {
    pub fn new() -> Self {
        VectorStoreRequest {
            file_ids: None,
            name: None,
            expires_after: None,
            chunking_strategy: None,
            metadata: None,
        }
    }

    pub fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn expires_after(mut self, expires_after: Value) -> Self {
        self.expires_after = Some(expires_after);
        self
    }

    pub fn chunking_strategy(mut self, chunking_strategy: Value) -> Self {
        self.chunking_strategy = Some(chunking_strategy);
        self
    }

    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl<'a> VectorsApi<'a> {
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        VectorsApi { client, base_url }
    }

    pub async fn create_vector_store(&self, request: VectorStoreRequest) -> Result<Value, OpenAIError> {
        let url = format!("{}/vector_stores", self.base_url);
        let mut body = serde_json::Map::new();

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

        let response = self.client.post(&url, &Value::Object(body)).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

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

    pub async fn retrieve_vector_store(&self, vector_store_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/vector_stores/{}", self.base_url, vector_store_id);
        let response = self.client.get(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

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

    pub async fn delete_vector_store(&self, vector_store_id: &str) -> Result<Value, OpenAIError> {
        let url = format!("{}/vector_stores/{}", self.base_url, vector_store_id);
        let response = self.client.delete(&url).await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
}