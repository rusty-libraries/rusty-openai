use reqwest::{Client, Error, Response};
use serde_json::Value;

pub struct RequestClient {
    pub client: Client,
    pub api_key: String,
}

impl RequestClient {
    pub fn new(api_key: &str) -> Self {
        RequestClient {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        self.client.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
    }

    pub async fn post(&self, url: &str, body: &Value) -> Result<Response, Error> {
        self.client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
    }
}