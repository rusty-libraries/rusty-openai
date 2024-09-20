use crate::{error_handling::OpenAIResult, extend_form_text_fields, openai::OpenAI};
use reqwest::multipart;
use serde_json::Value;
use tokio::fs;

/// AudioApi struct to interact with the audio transcription and translation API.
pub struct AudioApi<'a>(pub(crate) &'a OpenAI);

impl<'a> AudioApi<'a> {
    /// Transcribe an audio file using the specified model.
    pub async fn transcribe(
        &self,
        model: &str,                   // The transcription model to use
        file_path: &str,               // Path to the audio file
        prompt: Option<&str>,          // Optional prompt to guide transcription
        response_format: Option<&str>, // Optional response format (e.g., "text", "json")
        temperature: Option<f64>,      // Optional temperature setting for response generation
        language: Option<&str>,        // Optional language hint for the transcription
    ) -> OpenAIResult<Value> {
        let url = format!("{}/audio/transcriptions", self.0.base_url);

        // Open the audio file asynchronously
        let buffer = fs::read(file_path).await?;

        // Create multipart form with required fields
        let file_part = multipart::Part::bytes(buffer)
            .file_name(file_path.to_string())
            .mime_str("audio/mpeg")?;

        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("file", file_part);

        extend_form_text_fields!(form, prompt, response_format, temperature, language);

        // Make HTTP POST request to the transcription API
        self.0.post_form(&url, form).await
    }

    /// Translate an audio file using the specified model.
    pub async fn translate(
        &self,
        model: &str,                   // The translation model to use
        file_path: &str,               // Path to the audio file
        prompt: Option<&str>,          // Optional prompt to guide translation
        response_format: Option<&str>, // Optional response format (e.g., "text", "json")
        temperature: Option<f64>,      // Optional temperature setting for response generation
    ) -> OpenAIResult<Value> {
        let url = format!("{}/audio/translations", self.0.base_url);

        // Open the audio file asynchronously
        let buffer = fs::read(file_path).await?;

        // Create multipart form with required fields
        let file_part = multipart::Part::bytes(buffer)
            .file_name(file_path.to_string())
            .mime_str("audio/mpeg")?;

        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("file", file_part);

        extend_form_text_fields!(form, prompt, response_format, temperature);

        // Make HTTP POST request to the translation API
        self.0.post_form(&url, form).await
    }
}
