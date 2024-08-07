use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::Value;
use reqwest::multipart;
use tokio::io::AsyncReadExt;

/// AudioApi struct to interact with the audio transcription and translation API.
pub struct AudioApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

impl<'a> AudioApi<'a> {
    /// Create a new instance of AudioApi.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        AudioApi { client, base_url }
    }

    /// Transcribe an audio file using the specified model.
    pub async fn transcribe(
        &self,
        model: &str,                // The transcription model to use
        file_path: &str,            // Path to the audio file
        prompt: Option<&str>,       // Optional prompt to guide transcription
        response_format: Option<&str>, // Optional response format (e.g., "text", "json")
        temperature: Option<f64>,   // Optional temperature setting for response generation
        language: Option<&str>      // Optional language hint for the transcription
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/audio/transcriptions", self.base_url);

        // Open the audio file asynchronously
        let mut file = tokio::fs::File::open(file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        // Create multipart form with required fields
        let file_part = multipart::Part::bytes(buffer)
            .file_name(file_path.to_string())
            .mime_str("audio/mpeg")?;
        
        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("file", file_part);

        // Optionally add provided parameters to the form
        if let Some(prompt) = prompt {
            form = form.text("prompt", prompt.to_string());
        }
        if let Some(response_format) = response_format {
            form = form.text("response_format", response_format.to_string());
        }
        if let Some(temperature) = temperature {
            form = form.text("temperature", temperature.to_string());
        }
        if let Some(language) = language {
            form = form.text("language", language.to_string());
        }

        // Make HTTP POST request to the transcription API
        let response = self.client.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .multipart(form)
            .send()
            .await?;

        // Parse and return JSON response
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Translate an audio file using the specified model.
    pub async fn translate(
        &self,
        model: &str,                // The translation model to use
        file_path: &str,            // Path to the audio file
        prompt: Option<&str>,       // Optional prompt to guide translation
        response_format: Option<&str>, // Optional response format (e.g., "text", "json")
        temperature: Option<f64>    // Optional temperature setting for response generation
    ) -> Result<Value, OpenAIError> {
        let url = format!("{}/audio/translations", self.base_url);

        // Open the audio file asynchronously
        let mut file = tokio::fs::File::open(file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        // Create multipart form with required fields
        let file_part = multipart::Part::bytes(buffer)
            .file_name(file_path.to_string())
            .mime_str("audio/mpeg")?;
        
        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("file", file_part);

        // Optionally add provided parameters to the form
        if let Some(prompt) = prompt {
            form = form.text("prompt", prompt.to_string());
        }
        if let Some(response_format) = response_format {
            form = form.text("response_format", response_format.to_string());
        }
        if let Some(temperature) = temperature {
            form = form.text("temperature", temperature.to_string());
        }

        // Make HTTP POST request to the translation API
        let response = self.client.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .multipart(form)
            .send()
            .await?;

        // Parse and return JSON response
        let json: Value = response.json().await?;
        Ok(json)
    }
}