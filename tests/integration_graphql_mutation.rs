use async_graphql::{Schema, EmptySubscription};
use project_management::{config::owner_data::OwnerData, handler::graphql_handler::Mutation};
use rocket::tokio;


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

    // Parse the JSON string into a serde_json::Value
    let json_value: JsonValue = serde_json::from_str(&response_json).expect("Failed to parse JSON");

    let data_json = &json_value["data"]["createOwner"]["name"];

    let read_filedata: Vec<Owner> = read_data("owner.json").unwrap();
    let last_index = read_filedata.len() - 1;
    let file_name_data = &read_filedata[last_index].name;

    assert_eq!(data_json, file_name_data)

    // assert_eq!()
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

    // Parse the JSON string into a serde_json::Value
    let json_value: JsonValue = serde_json::from_str(&response_json).expect("Failed to parse JSON");

    let binding = json_value["data"]["createProject"]["id"].to_string();
    let data_json = binding.trim();

    let read_filedata: Vec<Project> = read_data("project.json").unwrap();
    let last_index = read_filedata.len() - 1;
    let file_name_data = read_filedata[last_index]._id.clone();
    println!(
        "{:?}\n{:?}",
        Some(data_json.trim_matches('"')),
        file_name_data
    );
    assert_eq!(Some(data_json.trim_matches('"')), file_name_data.as_deref());
    // assert_eq!()
}
