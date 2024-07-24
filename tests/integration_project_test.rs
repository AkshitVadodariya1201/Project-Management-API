use project_management::config::json_read_write::project_add_json;
use project_management::schema::project_schema::Status::*;
use project_management::{
    config::{json_read_write::project_read_json_data, project_data::ProjectData},
    schema::project_schema::Project,
};
use rocket::http::Status;

#[test]
pub fn for_create_project() {
    let project_create = ProjectData::init();

    let check_data = Project {
        _id: None,
        owner_id: "b15cc5e2-91c1-472c-a499-5e875117812d".to_string(),
        name: "Selling".to_string(),
        description: "Sell Cars".to_string(),
        status: NotStarted,
    };

    let data_come = project_create.create_project(check_data);
    let mut data_json = project_read_json_data("JSON/project.json");
    let last_index = data_json.len() - 1;

    let check = data_json[last_index].clone();
    data_json.remove(last_index);
    project_add_json(data_json, "JSON/project.json");
    assert_eq!(data_come, Ok(check));
}

#[test]
pub fn for_get_project() {
    let project = ProjectData::init();
    let get_project = project.get_projects();
    let check_data = project_read_json_data("JSON/project.json");
    assert_eq!(get_project, Ok(check_data));
}

#[test]
fn for_get_single_project() {
    let owner = ProjectData::init();
    let get_data = owner.single_project(&"d0c7d1b6-4690-4d68-9a01-b9cd99d181a5".to_string());
    let owner = Project {
        _id: Some("d0c7d1b6-4690-4d68-9a01-b9cd99d181a5".to_string()),
        owner_id: "859dd8ab-f058-473f-8fb6-75b797265ece".to_string(),
        name: "Buying".to_string(),
        description: "Buy Cars".to_string(),
        status: InProgress,
    };
    assert_eq!(get_data, Ok(owner));

    let project = ProjectData::init();
    let get_data = project.single_project(&"000----000---000".to_string());
    assert_eq!(get_data, Err(Status::NotFound));
}
