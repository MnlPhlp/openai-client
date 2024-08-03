use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::administration::project_user::CreateProjectUserParameters;
use crate::v1::resources::administration::project_user::ModifyProjectUserParameters;
use crate::v1::resources::administration::project_user::ProjectUser;
use crate::v1::resources::shared::DeletedObject;
use crate::v1::resources::shared::ListResponse;
use crate::v1::resources::shared::SimpleListParameters;

use super::Administration;

pub struct ProjectUsers<'a> {
    pub administration: &'a Administration<'a>,
}

impl Administration<'_> {
    /// Manage users within a project, including adding, updating roles, and removing users.
    pub fn project_users(&self) -> ProjectUsers {
        ProjectUsers {
            administration: self,
        }
    }
}

impl ProjectUsers<'_> {
    /// Returns a list of users in the project.
    pub async fn list(
        &self,
        project_id: &str,
        query: Option<SimpleListParameters>,
    ) -> Result<ListResponse<ProjectUser>, APIError> {
        let response = self
            .administration
            .client
            .get_with_query(
                &format!("/organization/projects/{}/users", project_id),
                &query,
            )
            .await?;

        let list_project_users_response: ListResponse<ProjectUser> = format_response(response)?;

        Ok(list_project_users_response)
    }

    /// Retrieves a user in the project.
    pub async fn retrieve(&self, project_id: &str, user_id: &str) -> Result<ProjectUser, APIError> {
        let response = self
            .administration
            .client
            .get(&format!(
                "/organization/projects/{}/users/{}",
                project_id, user_id
            ))
            .await?;

        let project_user: ProjectUser = format_response(response)?;

        Ok(project_user)
    }

    /// Adds a user to the project.
    pub async fn create(
        &self,
        project_id: &str,
        parameters: CreateProjectUserParameters,
    ) -> Result<ProjectUser, APIError> {
        let response = self
            .administration
            .client
            .post(
                &format!("/organization/projects/{}/users", project_id),
                &parameters,
            )
            .await?;

        let project_user: ProjectUser = format_response(response.data)?;

        Ok(project_user)
    }

    /// Modifies a user's role in the project.
    pub async fn modify(
        &self,
        project_id: &str,
        user_id: &str,
        parameters: ModifyProjectUserParameters,
    ) -> Result<ProjectUser, APIError> {
        let response = self
            .administration
            .client
            .post(
                &format!("/organization/projects/{}/users/{}", project_id, user_id),
                &parameters,
            )
            .await?;

        let project_user: ProjectUser = format_response(response.data)?;

        Ok(project_user)
    }

    /// Deletes a user from the project.
    pub async fn delete(&self, project_id: &str, user_id: &str) -> Result<DeletedObject, APIError> {
        let response = self
            .administration
            .client
            .delete(&format!(
                "/organization/projects/{}/users/{}",
                project_id, user_id
            ))
            .await?;

        let deleted_object: DeletedObject = format_response(response)?;

        Ok(deleted_object)
    }
}
