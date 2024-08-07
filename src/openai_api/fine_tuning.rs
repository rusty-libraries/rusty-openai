use crate::request_client::RequestClient;
use crate::error_handling::OpenAIError;
use serde_json::{json, Value};

/// FineTuningApi struct to interact with the fine-tuning endpoints of the API.
pub struct FineTuningApi<'a> {
    client: &'a RequestClient,  // Reference to the HTTP client
    base_url: &'a str,          // Base URL for the API
}

impl<'a> FineTuningApi<'a> {
    /// Create a new instance of FineTuningApi.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the RequestClient.
    /// * `base_url` - The base URL for the API.
    pub fn new(client: &'a RequestClient, base_url: &'a str) -> Self {
        FineTuningApi { client, base_url }
    }

    /// Create a new fine-tuning job with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to be fine-tuned.
    /// * `training_file` - The file containing training data.
    /// * `validation_file` - Optional validation data file.
    /// * `n_epochs` - Optional number of training epochs.
    /// * `batch_size` - Optional batch size for training.
    /// * `learning_rate_multiplier` - Optional learning rate multiplier.
    /// * `prompt_loss_weight` - Optional weight for the prompt loss.
    /// * `compute_classification_metrics` - Optional flag to compute classification metrics.
    /// * `classification_n_classes` - Optional number of classes for classification.
    /// * `classification_positive_class` - Optional positive class for classification.
    /// * `classification_betas` - Optional betas for classification metrics.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn create_fine_tuning_job(
        &self,
        model: &str,                               // Model to be fine-tuned
        training_file: &str,                       // Path to training data file
        validation_file: Option<&str>,             // Optional validation data file
        n_epochs: Option<u32>,                     // Optional number of epochs for training
        batch_size: Option<u32>,                   // Optional batch size for training
        learning_rate_multiplier: Option<f64>,     // Optional learning rate multiplier
        prompt_loss_weight: Option<f64>,           // Optional prompt loss weight
        compute_classification_metrics: Option<bool>, // Optional flag to compute classification metrics
        classification_n_classes: Option<u32>,     // Optional number of classes for classification
        classification_positive_class: Option<&str>,// Optional positive class for classification
        classification_betas: Option<Vec<f64>>     // Optional betas for classification metrics
    ) -> Result<Value, OpenAIError> {
        // Construct the full URL for the fine-tuning jobs endpoint.
        let url = format!("{}/fine-tuning/jobs", self.base_url);

        // Initialize a JSON map to build the request body.
        let mut body = serde_json::Map::new();

        // Insert required fields into the JSON map.
        body.insert("model".to_string(), json!(model));
        body.insert("training_file".to_string(), json!(training_file));

        // Insert optional fields if they are provided.
        if let Some(validation_file) = validation_file {
            body.insert("validation_file".to_string(), json!(validation_file));
        }
        if let Some(n_epochs) = n_epochs {
            body.insert("n_epochs".to_string(), json!(n_epochs));
        }
        if let Some(batch_size) = batch_size {
            body.insert("batch_size".to_string(), json!(batch_size));
        }
        if let Some(learning_rate_multiplier) = learning_rate_multiplier {
            body.insert("learning_rate_multiplier".to_string(), json!(learning_rate_multiplier));
        }
        if let Some(prompt_loss_weight) = prompt_loss_weight {
            body.insert("prompt_loss_weight".to_string(), json!(prompt_loss_weight));
        }
        if let Some(compute_classification_metrics) = compute_classification_metrics {
            body.insert("compute_classification_metrics".to_string(), json!(compute_classification_metrics));
        }
        if let Some(classification_n_classes) = classification_n_classes {
            body.insert("classification_n_classes".to_string(), json!(classification_n_classes));
        }
        if let Some(classification_positive_class) = classification_positive_class {
            body.insert("classification_positive_class".to_string(), json!(classification_positive_class));
        }
        if let Some(classification_betas) = classification_betas {
            body.insert("classification_betas".to_string(), json!(classification_betas));
        }

        // Send a POST request to the fine-tuning jobs endpoint with the request body.
        let response = self.client.post(&url, &Value::Object(body)).await?;

        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }

    /// List all fine-tuning jobs.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.

    pub async fn list_fine_tuning_jobs(&self) -> Result<Value, OpenAIError> {
        // Construct the full URL for listing fine-tuning jobs.
        let url = format!("{}/fine-tuning/jobs", self.base_url);
        
        // Send a GET request to the fine-tuning jobs endpoint.
        let response = self.client.get(&url).await?;
        
        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }

    /// Retrieve information about a specific fine-tuning job.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The ID of the fine-tuning job to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as `serde_json::Value` on success,
    /// or an OpenAIError on failure.
    pub async fn retrieve_fine_tuning_job(&self, job_id: &str) -> Result<Value, OpenAIError> {
        // Construct the full URL for retrieving a specific fine-tuning job.
        let url = format!("{}/fine-tuning/jobs/{}", self.base_url, job_id);
        
        // Send a GET request to the specific fine-tuning job endpoint.
        let response = self.client.get(&url).await?;
        
        // Parse the JSON response body.
        let json: Value = response.json().await?;
        
        // Return the parsed JSON response.
        Ok(json)
    }
}