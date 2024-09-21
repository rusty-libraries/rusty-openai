use crate::{error_handling::OpenAIResult, openai::OpenAI};
use serde::Serialize;
use serde_json::Value;

/// [`ProjectsApi`] struct to interact with the projects endpoints of the API.
#[allow(dead_code)]
pub struct ProjectsApi<'a>(pub(crate) &'a OpenAI<'a>);

#[allow(dead_code)]
#[derive(Serialize)]
struct CreateProjectRequest<'a> {
    /// The friendly name of the project
    name: &'a str,

    /// Optional description of the business, project, or use case
    #[serde(skip_serializing_if = "Option::is_none")]
    app_use_case: Option<&'a str>,

    /// Optional business URL or social media link
    #[serde(skip_serializing_if = "Option::is_none")]
    business_website: Option<&'a str>,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct CreateProjectUserRequest<'a> {
    /// The ID of the user
    user_id: &'a str,

    /// The role of the user (owner or member)
    role: &'a str,
}

impl<'a> ProjectsApi<'a> {
    /// List projects within an organization.
    ///
    /// # Arguments
    ///
    /// * `limit` - Optional limit on the number of objects to return (1-100, default 20).
    /// * `after` - Optional cursor for pagination.
    /// * `include_archived` - Optional flag to include archived projects.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn list_projects(
        &self,
        limit: Option<u8>,
        after: Option<&str>,
        include_archived: Option<bool>,
    ) -> OpenAIResult<Value> {
        let mut url = "/organization/projects".to_string();
        let mut query_params = Vec::new();

        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(after) = after {
            query_params.push(format!("after={}", after));
        }
        if let Some(include_archived) = include_archived {
            query_params.push(format!("include_archived={}", include_archived));
        }

        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        self.0.get(&url).await
    }

    /// Create a new project in the organization.
    ///
    /// # Arguments
    ///
    /// * `name` - The friendly name of the project.
    /// * `app_use_case` - Optional description of the business, project, or use case.
    /// * `business_website` - Optional business URL or social media link.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn create_project(
        &self,
        name: &str,
        app_use_case: Option<&str>,
        business_website: Option<&str>,
    ) -> OpenAIResult<Value> {
        let body = CreateProjectRequest {
            name,
            app_use_case,
            business_website,
        };

        self.0.post_json("/organization/projects", &body).await
    }

    /// Retrieve information about a specific project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn retrieve_project(&self, project_id: &str) -> OpenAIResult<Value> {
        let url = format!("/organization/projects/{}", project_id);
        self.0.get(&url).await
    }

    /// Modify an existing project in the organization.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to modify.
    /// * `name` - The updated name of the project.
    /// * `app_use_case` - Optional updated description of the business, project, or use case.
    /// * `business_website` - Optional updated business URL or social media link.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn modify_project(
        &self,
        project_id: &str,
        name: &str,
        app_use_case: Option<&str>,
        business_website: Option<&str>,
    ) -> OpenAIResult<Value> {
        let body = CreateProjectRequest {
            name,
            app_use_case,
            business_website,
        };

        let url = format!("/organization/projects/{}", project_id);
        self.0.post_json(&url, &body).await
    }

    /// Archive a project in the organization.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to archive.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn archive_project(&self, project_id: &str) -> OpenAIResult<Value> {
        let url = format!("/organization/projects/{}/archive", project_id);
        self.0.post_json(&url, &serde_json::json!({})).await
    }

    /// List users in a project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project.
    /// * `limit` - Optional limit on the number of objects to return (1-100, default 20).
    /// * `after` - Optional cursor for pagination.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn list_project_users(
        &self,
        project_id: &str,
        limit: Option<u8>,
        after: Option<&str>,
    ) -> OpenAIResult<Value> {
        let mut url = format!("/organization/projects/{}/users", project_id);
        let mut query_params = Vec::new();

        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(after) = after {
            query_params.push(format!("after={}", after));
        }

        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        self.0.get(&url).await
    }

    /// Add a user to a project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project.
    /// * `user_id` - The ID of the user to add.
    /// * `role` - The role of the user (owner or member).
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn create_project_user(
        &self,
        project_id: &str,
        user_id: &str,
        role: &str,
    ) -> OpenAIResult<Value> {
        let body = CreateProjectUserRequest { user_id, role };
        let url = format!("/organization/projects/{}/users", project_id);
        self.0.post_json(&url, &body).await
    }

    /// Retrieve information about a specific user in a project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project.
    /// * `user_id` - The ID of the user.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn retrieve_project_user(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> OpenAIResult<Value> {
        let url = format!("/organization/projects/{}/users/{}", project_id, user_id);
        self.0.get(&url).await
    }

    /// Modify a user's role in a project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project.
    /// * `user_id` - The ID of the user.
    /// * `role` - The new role of the user (owner or member).
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn modify_project_user(
        &self,
        project_id: &str,
        user_id: &str,
        role: &str,
    ) -> OpenAIResult<Value> {
        let body = serde_json::json!({ "role": role });
        let url = format!("/organization/projects/{}/users/{}", project_id, user_id);
        self.0.post_json(&url, &body).await
    }

    /// Delete a user from a project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project.
    /// * `user_id` - The ID of the user to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON response as [`serde_json::Value`] on success, or an [`OpenAIError`][crate::error_handling::OpenAIError] on failure.
    pub async fn delete_project_user(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> OpenAIResult<Value> {
        let url = format!("/organization/projects/{}/users/{}", project_id, user_id);
        self.0.delete(&url).await
    }
}