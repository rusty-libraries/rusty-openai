use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};
use reqwest::multipart;
use tokio::io::AsyncReadExt;

/// ImagesApi struct to interact with the image generation, editing, and variation endpoints of the API.
pub struct ImagesApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

impl<'a> ImagesApi<'a> {
    /// Create a new instance of ImagesApi.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the RequestClient.
    /// * `base_url` - The base URL for the API.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        ImagesApi { client, base_url }
    }

    /// Generate an image based on the provided prompt and parameters.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The text prompt to generate the image from.
    /// * `model` - The name of the model to use for generating the image.
    /// * `size` - Optional size of the image.
    /// * `response_format` - Optional response format (e.g., `json`, `url`).
    /// * `n` - Optional number of images to generate.
    /// * `user` - Optional user ID.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn generate(
        &self,
        prompt: &str,                 // The text prompt to generate the image from
        model: &str,                  // The model to use for generating the image
        size: Option<&str>,           // Optional size of the image
        response_format: Option<&str>,// Optional response format
        n: Option<u64>,               // Optional number of images to generate
        user: Option<&str>            // Optional user ID
    ) -> Result<Value, OpenAIError> {
        // Construct the full URL for the image generation endpoint.
        let url = format!("{}/images/generations", self.base_url);
        
        // Initialize a JSON object to build the request body.
        let mut body = json!({
            "prompt": prompt,
            "model": model
        });

        // Insert optional fields if they are provided.
        if let Some(size) = size {
            if let Value::Object(map) = &mut body {
                map.insert("size".to_string(), json!(size));
            }
        }

        if let Some(response_format) = response_format {
            if let Value::Object(map) = &mut body {
                map.insert("response_format".to_string(), json!(response_format));
            }
        }

        if let Some(n) = n {
            if let Value::Object(map) = &mut body {
                map.insert("n".to_string(), json!(n));
            }
        }

        if let Some(user) = user {
            if let Value::Object(map) = &mut body {
                map.insert("user".to_string(), json!(user));
            }
        }

        // Send a POST request to the image generation endpoint with the request body.
        let response = self.client.post(&url, &body).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }

    /// Edit an existing image using the provided parameters and mask.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model to use for editing the image.
    /// * `image_path` - The local file path to the image.
    /// * `mask_path` - The local file path to the mask.
    /// * `prompt` - The text prompt to guide the editing.
    /// * `size` - Optional size of the edited image.
    /// * `response_format` - Optional response format (e.g., `json`, `url`).
    /// * `n` - Optional number of edited images to generate.
    /// * `user` - Optional user ID.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn edit(
        &self,
        model: &str,                  // The model to use for editing the image
        image_path: &str,             // Local file path to the image
        mask_path: &str,              // Local file path to the mask
        prompt: &str,                 // Text prompt to guide the editing
        size: Option<&str>,           // Optional size of the edited image
        response_format: Option<&str>,// Optional response format
        n: Option<u64>,               // Optional number of edited images to generate
        user: Option<&str>            // Optional user ID
    ) -> Result<Value, OpenAIError> {
        // Construct the full URL for the image editing endpoint.
        let url = format!("{}/images/edits", self.base_url);

        // Open and read the image file asynchronously.
        let mut image = tokio::fs::File::open(image_path).await?;
        let mut image_buffer = Vec::new();
        image.read_to_end(&mut image_buffer).await?;
        let image_part = multipart::Part::bytes(image_buffer)
            .file_name(image_path.to_string())
            .mime_str("image/png")?;

        // Open and read the mask file asynchronously.
        let mut mask = tokio::fs::File::open(mask_path).await?;
        let mut mask_buffer = Vec::new();
        mask.read_to_end(&mut mask_buffer).await?;
        let mask_part = multipart::Part::bytes(mask_buffer)
            .file_name(mask_path.to_string())
            .mime_str("image/png")?;

        // Initialize a multipart form to build the request body.
        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("image", image_part)
            .part("mask", mask_part)
            .text("prompt", prompt.to_string());

        // Insert optional fields if they are provided.
        if let Some(size) = size {
            form = form.text("size", size.to_string());
        }

        if let Some(response_format) = response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(n) = n {
            form = form.text("n", n.to_string());
        }

        if let Some(user) = user {
            form = form.text("user", user.to_string());
        }

        // Send a POST request to the image editing endpoint with the multipart form.
        let response = self.client.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .multipart(form)
            .send()
            .await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }

    /// Create variations of an existing image using the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model to use for generating variations.
    /// * `image_path` - The local file path to the image.
    /// * `size` - Optional size of the variation images.
    /// * `response_format` - Optional response format (e.g., `json`, `url`).
    /// * `n` - Optional number of variation images to generate.
    /// * `user` - Optional user ID.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn variation(
        &self,
        model: &str,                  // The model to use for generating variations
        image_path: &str,             // Local file path to the image
        size: Option<&str>,           // Optional size of the variation images
        response_format: Option<&str>,// Optional response format
        n: Option<u64>,               // Optional number of variation images to generate
        user: Option<&str>            // Optional user ID
    ) -> Result<Value, OpenAIError> {
        // Construct the full URL for the image variations endpoint.
        let url = format!("{}/images/variations", self.base_url);

        // Open and read the image file asynchronously.
        let mut image = tokio::fs::File::open(image_path).await?;
        let mut buffer = Vec::new();
        image.read_to_end(&mut buffer).await?;
        let image_part = multipart::Part::bytes(buffer)
            .file_name(image_path.to_string())
            .mime_str("image/png")?;

        // Initialize a multipart form to build the request body.
        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("image", image_part);

        // Insert optional fields if they are provided.
        if let Some(size) = size {
            form = form.text("size", size.to_string());
        }

        if let Some(response_format) = response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(n) = n {
            form = form.text("n", n.to_string());
        }

        if let Some(user) = user {
            form = form.text("user", user.to_string());
        }

        // Send a POST request to the image variations endpoint with the multipart form.
        let response = self.client.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .multipart(form)
            .send()
            .await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}