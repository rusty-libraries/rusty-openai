use crate::{error_handling::OpenAIResult, openai::OpenAI};
use serde::Serialize;
use serde_json::Value;

/// [`FineTuningApi`] struct to interact with the fine-tuning endpoints of the API.
pub struct FineTuningApi<'a>(pub(crate) &'a OpenAI<'a>);

#[derive(Serialize)]
struct FineTuningRequest<'a> {
    /// Model to be fine-tuned
    model: &'a str,

    /// Path to training data file
    training_file: &'a str,

    /// Optional validation data file
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_file: Option<&'a str>,

    /// Optional number of epochs for training
    #[serde(skip_serializing_if = "Option::is_none")]
    n_epochs: Option<u32>,

    /// Optional batch size for training
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<u32>,

    /// Optional learning rate multiplier
    #[serde(skip_serializing_if = "Option::is_none")]
    learning_rate_multiplier: Option<f64>,

    /// Optional prompt loss weight
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_loss_weight: Option<f64>,

    /// Optional flag to compute classification metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_classification_metrics: Option<bool>,

    /// Optional number of classes for classification
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_n_classes: Option<u32>,

    /// Optional positive class for classification
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_positive_class: Option<&'a str>,

    /// Optional betas for classification metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_betas: Option<Vec<f64>>,
}

impl<'a> FineTuningApi<'a> {
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
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn create_fine_tuning_job(
        &self,
        model: &str,                                  // Model to be fine-tuned
        training_file: &str,                          // Path to training data file
        validation_file: Option<&str>,                // Optional validation data file
        n_epochs: Option<u32>,                        // Optional number of epochs for training
        batch_size: Option<u32>,                      // Optional batch size for training
        learning_rate_multiplier: Option<f64>,        // Optional learning rate multiplier
        prompt_loss_weight: Option<f64>,              // Optional prompt loss weight
        compute_classification_metrics: Option<bool>, // Optional flag to compute classification metrics
        classification_n_classes: Option<u32>, // Optional number of classes for classification
        classification_positive_class: Option<&str>, // Optional positive class for classification
        classification_betas: Option<Vec<f64>>, // Optional betas for classification metrics
    ) -> OpenAIResult<Value> {
        // Initialize a JSON map to build the request body.
        let body = FineTuningRequest {
            model,
            training_file,
            validation_file,
            n_epochs,
            batch_size,
            learning_rate_multiplier,
            prompt_loss_weight,
            compute_classification_metrics,
            classification_n_classes,
            classification_positive_class,
            classification_betas,
        };

        // Send a POST request to the fine-tuning jobs endpoint with the request body.
        self.0.post_json("/fine-tuning/jobs", &body).await
    }

    /// List all fine-tuning jobs.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.

    pub async fn list_fine_tuning_jobs(&self) -> OpenAIResult<Value> {
        // Send a GET request to the fine-tuning jobs endpoint.
        self.0.get("/fine-tuning/jobs").await
    }

    /// Retrieve information about a specific fine-tuning job.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The ID of the fine-tuning job to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn retrieve_fine_tuning_job(&self, job_id: &str) -> OpenAIResult<Value> {
        // Construct the full URL for retrieving a specific fine-tuning job.
        let url = format!("/fine-tuning/jobs/{job_id}");

        // Send a GET request to the specific fine-tuning job endpoint.
        self.0.get(&url).await
    }
}
