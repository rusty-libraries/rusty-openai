use crate::openai_api::embeddings::EmbeddingsApi;
use crate::request_client::RequestClient;
use crate::openai_api::{
    client::ClientApi,
    completion::CompletionsApi,
    audio::AudioApi,
    images::ImagesApi,
    fine_tuning::FineTuningApi,
    moderations::ModerationApi
};

pub struct OpenAI {
    client: RequestClient,
    base_url: String,
}

impl OpenAI {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        let default_base_url = "https://api.openai.com/v1";
        OpenAI {
            client: RequestClient::new(api_key),
            base_url: if base_url.is_empty() { default_base_url.to_string() } else { base_url.to_string() },
        }
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.to_string();
    }

    pub fn client(&self) -> ClientApi {
        ClientApi::new(&self.client, &self.base_url)
    }

    pub fn completions(&self) -> CompletionsApi {
        CompletionsApi::new(&self.client, &self.base_url)
    }

    pub fn audio(&self) -> AudioApi {
        AudioApi::new(&self.client, &self.base_url)
    }

    pub fn images(&self) -> ImagesApi {
        ImagesApi::new(&self.client, &self.base_url)
    }

    pub fn fine_tuning(&self) -> FineTuningApi {
        FineTuningApi::new(&self.client, &self.base_url)
    }

    pub fn moderation(&self) -> ModerationApi {
        ModerationApi::new(&self.client, &self.base_url)
    }

    pub fn embeddings(&self) -> EmbeddingsApi {
        EmbeddingsApi::new(&self.client, &self.base_url)
    }
}