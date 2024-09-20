use crate::{
    error_handling::OpenAIResult,
    openai_api::{
        assistants::AssistantsApi, audio::AudioApi, client::ClientApi, completion::CompletionsApi,
        embeddings::EmbeddingsApi, fine_tuning::FineTuningApi, images::ImagesApi,
        moderations::ModerationApi, threads::ThreadsApi, vectors::VectorsApi,
    },
};
use reqwest::{multipart::Form, Client};
use serde::{de::DeserializeOwned, Serialize};

pub struct OpenAI {
    pub(crate) client: Client,
    authorization: String,
    pub(crate) base_url: String,
}

impl OpenAI {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        let default_base_url = "https://api.openai.com/v1";

        Self {
            client: Client::new(),
            authorization: format!("Bearer {api_key}"),
            base_url: {
                if base_url.is_empty() {
                    default_base_url
                } else {
                    base_url
                }
            }
            .to_string(),
        }
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, url: &str) -> OpenAIResult<T> {
        Ok(self
            .client
            .get(format!("{}{url}", self.base_url))
            .header("Authorization", &self.authorization)
            .send()
            .await?
            .json()
            .await?)
    }

    pub(crate) async fn post_json<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
    ) -> OpenAIResult<T> {
        Ok(self
            .client
            .post(format!("{}{url}", self.base_url))
            .header("Authorization", &self.authorization)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?
            .json()
            .await?)
    }

    pub(crate) async fn post_form<T: DeserializeOwned>(
        &self,
        url: &str,
        form: Form,
    ) -> OpenAIResult<T> {
        Ok(self
            .client
            .post(format!("{}{url}", self.base_url))
            .header("Authorization", &self.authorization)
            .multipart(form)
            .send()
            .await?
            .json()
            .await?)
    }

    pub(crate) async fn delete<T: DeserializeOwned>(&self, url: &str) -> OpenAIResult<T> {
        Ok(self
            .client
            .delete(format!("{}{url}", self.base_url))
            .header("Authorization", &self.authorization)
            .send()
            .await?
            .json()
            .await?)
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.to_string();
    }

    pub const fn client(&self) -> ClientApi {
        ClientApi(self)
    }

    pub const fn completions(&self) -> CompletionsApi {
        CompletionsApi(self)
    }

    pub const fn audio(&self) -> AudioApi {
        AudioApi(self)
    }

    pub const fn images(&self) -> ImagesApi {
        ImagesApi(self)
    }

    pub const fn fine_tuning(&self) -> FineTuningApi {
        FineTuningApi(self)
    }

    pub const fn moderation(&self) -> ModerationApi {
        ModerationApi(self)
    }

    pub const fn embeddings(&self) -> EmbeddingsApi {
        EmbeddingsApi(self)
    }

    pub const fn assistants(&self) -> AssistantsApi {
        AssistantsApi(self)
    }

    pub const fn threads(&self) -> ThreadsApi {
        ThreadsApi(self)
    }

    pub const fn vectors(&self) -> VectorsApi {
        VectorsApi(self)
    }
}
