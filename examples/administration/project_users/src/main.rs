use openai_dive::v1::api::Client;
use openai_dive::v1::resources::administration::project_user::ModifyProjectUserParametersBuilder;
use openai_dive::v1::resources::administration::project_user::ProjectUserRole;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let project_users = client
        .administration()
        .project_users()
        .list("proj_XXX", None)
        .await?;

    println!("{:#?}", project_users);

    let project_user = client
        .administration()
        .project_users()
        .retrieve("proj_XXX", "user-XXX")
        .await?;

    println!("{:#?}", project_user);

    let parameters = ModifyProjectUserParametersBuilder::default()
        .role(ProjectUserRole::Owner)
        .build()?;

    let modified_project_user = client
        .administration()
        .project_users()
        .modify("proj_XXX", "user-XXX", parameters)
        .await?;

    println!("{:#?}", modified_project_user);

    Ok(())
}
