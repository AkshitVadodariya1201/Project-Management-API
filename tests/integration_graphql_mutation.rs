use async_graphql::{EmptySubscription, Schema};
use project_management::config::json_read_write::{owner_read_json_data, project_read_json_data};
use project_management::config::project_data::ProjectData;
use project_management::handler::graphql_handler::{Mutation, Query};
use project_management::schema::project_schema::Project;
use project_management::{config::owner_data::OwnerData, schema::owner_schema::Owner};
use rocket::tokio;
use serde_json::Value as JsonValue; // Add this line to import JsonValue

#[tokio::test]
async fn test_for_create_owner() {
    let owner_db = OwnerData::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(owner_db)
        .finish();

    let response = schema
        .execute(
            r#"mutation{
              createOwner(
              input:{
                name:"bot",
                email:"bot1232gmail.com",
                phone:"1111111111"
              }
              ){
                id
                name
                email
                phone
              }
            }"#,
        )
        .await
        .into_result()
        .unwrap();
    let response_json =
        serde_json::to_string_pretty(&response).expect("Failed to serialize GraphQL response");

    // Parse the JSON string into a serde_json::Value with explicit type annotation
    let json_value: JsonValue = serde_json::from_str(&response_json).expect("Failed to parse JSON");

    let data_json = json_value["data"]["createOwner"]["name"]
        .as_str()
        .expect("Failed to extract name");

    let read_filedata: Vec<Owner> = owner_read_json_data("owner.json");
    let last_index = read_filedata.len() - 1;
    let file_name_data = &read_filedata[last_index].name;

    assert_eq!(data_json, file_name_data);
}

#[tokio::test]
async fn test_for_create_project() {
    let owner_db = ProjectData::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(owner_db)
        .finish();

    let response = schema
        .execute(
            r#"mutation{
            createProject(input:{
              ownerId: "000000",
              name: "bot test",
              description: "test sell",
              status: "test Started"
            })
            {
              id
              ownerId
              description
              name
              status
              
            }
          }"#,
        )
        .await
        .into_result()
        .unwrap();
    let response_json =
        serde_json::to_string_pretty(&response).expect("Failed to serialize GraphQL response");

    // Parse the JSON string into a serde_json::Value with explicit type annotation
    let json_value: JsonValue = serde_json::from_str(&response_json).expect("Failed to parse JSON");

    let binding = json_value["data"]["createProject"]["id"]
        .as_str()
        .expect("Failed to extract id");

    let read_filedata: Vec<Project> = project_read_json_data("project.json");
    let last_index = read_filedata.len() - 1;
    let file_name_data = read_filedata[last_index]._id.clone();
    println!(
        "{:?}\n{:?}",
        Some(binding.trim_matches('"')),
        file_name_data
    );
    assert_eq!(Some(binding.trim_matches('"')), file_name_data.as_deref());
}
