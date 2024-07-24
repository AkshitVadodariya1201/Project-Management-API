use project_management::schema::project_schema::Status::*;
use project_management::{
    config::json_read_write::{
        owner_add_json, owner_read_json_data, project_add_json, project_read_json_data,
    },
    schema::{owner_schema::Owner, project_schema::Project},
};
use uuid::Uuid;

#[test]
#[should_panic]
fn test_json() {
    let data = owner_read_json_data("JSON_Test/owner_test.json");

    let ckeck_data = Owner {
        _id: Some("0820e4ec-462d-4f61-a830-8573fa227a5d".to_string()),
        name: "Akshit Kumar Vadodariya".to_string(),
        email: "akshitvadodariya121@gmail.com".to_string(),
        phone: "741825296330".to_string(),
    };
    println!("{:?}", ckeck_data);
    assert_eq!(ckeck_data._id, data[0]._id);
}
// #[test]
// fn test_json_empty() {
//     let data = owner_read_json_data("JSON/owner.json");
//     println!("{:?}", data);
//     assert_eq!(data, None);
// }
// #[test]
// fn test_read_invalid_data() {
//     let invalid_json = "not a valid JSON";

//     let file = std::env::temp_dir().join("owner.json");
//     std::fs::write(&file, invalid_json).unwrap();

//     let result = owner_read_json_data("JSON/owner.json");

//     assert_eq!(result, None);
// }

// #[test]
// fn test_file_not_found() {
//     let result = read_data::<Owner>("non_existent_file.json");
//     assert_eq!(result, None);
// }

#[test]
#[should_panic]
fn test_write_owner() {
    let data = owner_read_json_data("JSON/owner.json");
    let uuid_id = Uuid::new_v4();
    let mut add_new_owner = Vec::new();
    let case1 = data.len();
    let check_data = Owner {
        _id: Some(uuid_id.to_string()),
        name: "Ak ".to_string(),
        email: "ak@gamil.com".to_string(),
        phone: "992773629".to_string(),
    };
    add_new_owner.push(check_data);
    owner_add_json(add_new_owner, "JSON_Test/owner_test.json");
    let new_data = owner_read_json_data("JSON/owner.json");
    // let file_path = "test_owner.json";
    // fs::remove_file("test_owner.json");
    assert_ne!(case1, new_data.len());
}

#[test]
#[should_panic]
fn test_write_project() {
    let data = project_read_json_data("JSON_Test/project_test.json");
    let uuid_id = Uuid::new_v4();
    let mut add_new_project = Vec::new();
    let case1 = data.len();
    let check_data = Project {
        _id: Some(uuid_id.to_string()),
        owner_id: "0820e4ec-462d-4f61-a830-8573fa227a5d".to_string(),
        name: "Selling".to_string(),
        description: "Sell Cars".to_string(),
        status: NotStarted,
    };
    add_new_project.push(check_data);
    project_add_json(add_new_project, "JSON_Test/project_test.json");
    let new_data = project_read_json_data("JSON_Test/project_test.json");
    // let _file_path = "test.json";
    // fs::remove_file("test.json");
    assert_ne!(case1, new_data.len());
}
