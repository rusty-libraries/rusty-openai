use crate::{error_handling::OpenAIResult, extend_form_text_fields, openai::OpenAI};
use reqwest::multipart;
use serde::Serialize;
use serde_json::Value;
use tokio::fs;

/// [`ImagesApi`] struct to interact with the image generation, editing, and variation endpoints of the API.
pub struct ImagesApi<'a>(pub(crate) &'a OpenAI);

#[derive(Serialize)]
struct GenerateImageRequest<'a> {
    /// The text prompt to generate the image from
    prompt: &'a str,

    /// The model to use for generating the image
    model: &'a str,

    /// Optional size of the image
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<&'a str>,

    /// Optional response format
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<&'a str>,

    /// Optional number of images to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u64>,

    /// Optional user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a str>,
}

impl<'a> ImagesApi<'a> {
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn generate(
        &self,
        prompt: &str,                  // The text prompt to generate the image from
        model: &str,                   // The model to use for generating the image
        size: Option<&str>,            // Optional size of the image
        response_format: Option<&str>, // Optional response format
        n: Option<u64>,                // Optional number of images to generate
        user: Option<&str>,            // Optional user ID
    ) -> OpenAIResult<Value> {
        // Initialize a JSON object to build the request body.
        let body = GenerateImageRequest {
            prompt,
            model,
            size,
            response_format,
            n,
            user,
        };

        // Send a POST request to the image generation endpoint with the request body.
        self.0.post_json("/images/generations", &body).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn edit(
        &self,
        model: &str,                   // The model to use for editing the image
        image_path: &str,              // Local file path to the image
        mask_path: &str,               // Local file path to the mask
        prompt: &str,                  // Text prompt to guide the editing
        size: Option<&str>,            // Optional size of the edited image
        response_format: Option<&str>, // Optional response format
        n: Option<u64>,                // Optional number of edited images to generate
        user: Option<&str>,            // Optional user ID
    ) -> OpenAIResult<Value> {
        // Open and read the image file asynchronously.
        let image_buffer = fs::read(image_path).await?;
        let image_part = multipart::Part::bytes(image_buffer)
            .file_name(image_path.to_string())
            .mime_str("image/png")?;

        // Open and read the mask file asynchronously.
        let mask_buffer = fs::read(mask_path).await?;
        let mask_part = multipart::Part::bytes(mask_buffer)
            .file_name(mask_path.to_string())
            .mime_str("image/png")?;

        // Initialize a multipart form to build the request body.
        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("image", image_part)
            .part("mask", mask_part)
            .text("prompt", prompt.to_string());

        extend_form_text_fields!(form, size, response_format, n, user);

        // Send a POST request to the image editing endpoint with the multipart form.
        self.0.post_form("/images/edits", form).await
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn variation(
        &self,
        model: &str,                   // The model to use for generating variations
        image_path: &str,              // Local file path to the image
        size: Option<&str>,            // Optional size of the variation images
        response_format: Option<&str>, // Optional response format
        n: Option<u64>,                // Optional number of variation images to generate
        user: Option<&str>,            // Optional user ID
    ) -> OpenAIResult<Value> {
        // Open and read the image file asynchronously.
        let buffer = fs::read(image_path).await?;
        let image_part = multipart::Part::bytes(buffer)
            .file_name(image_path.to_string())
            .mime_str("image/png")?;

        // Initialize a multipart form to build the request body.
        let mut form = multipart::Form::new()
            .text("model", model.to_string())
            .part("image", image_part);

        extend_form_text_fields!(form, size, response_format, n, user);

        // Send a POST request to the image variations endpoint with the multipart form.
        self.0.post_form("/images/variations", form).await
    }
}
