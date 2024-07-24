use async_graphql::{EmptySubscription, Schema};
use project_management::{
    config::{
        json_read_write::{owner_read_json_data, project_read_json_data},
        owner_data::OwnerData,
        project_data::ProjectData,
    },
    handler::graphql_handler::{Mutation, Query},
    schema::{owner_schema::Owner, project_schema::Project},
};
use rocket::tokio;
use serde_json::Value as JsonValue;

#[tokio::test]
async fn test_for_owner() {
    let owner_db = OwnerData::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(owner_db)
        .finish();

    let response = schema
        .execute(
            r#"{
                owner(input:{id:"389287"}) {
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
    println!("{response_json}");

    // Parse the JSON string into a serde_json::Value
    let json_value: JsonValue = serde_json::from_str(&response_json).expect("Failed to parse JSON");

    let data_json = &json_value["data"]["owner"]["name"];

    let read_filedata: Vec<Owner> = owner_read_json_data("owner.json");
    let file_name_data = &read_filedata[0].name;

    assert_eq!(data_json, file_name_data)

    // assert_eq!()
}
#[tokio::test]
async fn test_for_get_owners() {
    let owner_db = OwnerData::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(owner_db)
        .finish();

    let response = schema
        .execute(
            r#"query{
                getOwners{
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

    // Parse the JSON string into a serde_json::Value
    let mut json_value: JsonValue =
        serde_json::from_str(&response_json).expect("Failed to parse JSON");

    println!("{}", json_value);
    let mut _check_data = &mut json_value["data"]["getOwners"];
    if let Some(array) = _check_data.as_array_mut() {
        for object in array {
            if let Some(id) = object.get("id") {
                object["_id"] = id.clone();
                object.as_object_mut().unwrap().remove("id");
            }
        }
    }
    let _file_data: Vec<Owner> = owner_read_json_data("owner.json");
    let read_file = &_file_data[0]._id;

    assert_eq!(*_check_data.to_string(), read_file.clone().unwrap());
}
#[tokio::test]
async fn test_for_project() {
    let owner_db = ProjectData::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(owner_db)
        .finish();

    let response = schema
        .execute(
            r#"{
                project(input:{id:"1111111"}){
                    id
                    ownerId
                    name
                    description
                    status
                }
            }"#,
        )
        .await
        .into_result()
        .unwrap();
    let response_json =
        serde_json::to_string_pretty(&response).expect("Failed to serialize GraphQL response");

    // Parse the JSON string into a serde_json::Value
    let json_value: JsonValue = serde_json::from_str(&response_json).expect("Failed to parse JSON");

    println!("{}", json_value);

    let data_json = &json_value["data"]["project"]["id"];

    let read_filedata: Vec<Project> = project_read_json_data("project.json");
    let file_name_data = &read_filedata[0]._id;

    assert_eq!(data_json.to_string(), file_name_data.clone().unwrap());

    // assert_eq!(data_json, file_name_data)

    // assert_eq!()
}
#[tokio::test]
async fn test_for_get_projects() {
    let owner_db = ProjectData::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(owner_db)
        .finish();

    let response = schema
        .execute(
            r#"query{
                getProjects{
                    id
                    ownerId
                    name
                    description
                    status
                }
            }"#,
        )
        .await
        .into_result()
        .unwrap();
    let response_json =
        serde_json::to_string_pretty(&response).expect("Failed to serialize GraphQL response");

    // Parse the JSON string into a serde_json::Value
    let mut json_value: JsonValue =
        serde_json::from_str(&response_json).expect("Failed to parse JSON");

    let read_filedata: Vec<Project> = project_read_json_data("project.json");

    let mut _check_data = &mut json_value["data"]["getProjects"];
    if let Some(array) = _check_data.as_array_mut() {
        for object in array {
            if let Some(id) = object.get("id") {
                object["_id"] = id.clone();
                object.as_object_mut().unwrap().remove("id");
            }
            if let Some(id) = object.get("ownerId") {
                object["owner_id"] = id.clone();
                object.as_object_mut().unwrap().remove("ownerId");
            }
        }
    }
    let data_json = &mut json_value["data"]["getProjects"];
    let file_name_data = &read_filedata[0]._id;
    assert_eq!(data_json.to_string(), file_name_data.clone().unwrap());

    // println!("{}\n{}",data_json,file_name_data); //make sure by printing those things
}
