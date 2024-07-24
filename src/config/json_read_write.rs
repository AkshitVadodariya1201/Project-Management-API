use std::{
    fs::{self, File},
    io::{Read, Write},
};

use crate::schema::{owner_schema::Owner, project_schema::Project};

pub fn owner_add_json(new_info: Vec<Owner>, path: &str) {
    let mut exist_data = File::open(path).unwrap();
    let mut data = String::new();
    exist_data.read_to_string(&mut data).unwrap();

    let exist_data_info: Vec<Owner> = serde_json::from_str(&data).expect("Failed to parse JSON");

    let mut update_data = Vec::new();
    update_data.extend_from_slice(&exist_data_info);
    update_data.extend_from_slice(&new_info);

    let mut file = fs::File::create(path).unwrap();
    let serialized = serde_json::to_string_pretty(&update_data).expect("Failed to serialize JSON");
    file.write_all(serialized.as_bytes())
        .expect("Failed to write file");
}

pub fn owner_read_json_data(path: &str) -> Vec<Owner> {
    let mut data_base = File::open(path).unwrap();
    let mut data = String::new();
    data_base.read_to_string(&mut data).unwrap();

    let venue_data: Vec<Owner> = serde_json::from_str(&data).expect("Failed to parse JSON");
    venue_data
}

pub fn project_add_json(new_info: Vec<Project>, path: &str) {
    let mut exist_data = File::open(path).unwrap();
    let mut data = String::new();
    exist_data.read_to_string(&mut data).unwrap();

    let exist_data_info: Vec<Project> = serde_json::from_str(&data).expect("Failed to parse JSON");

    let mut update_data = Vec::new();
    update_data.extend_from_slice(&exist_data_info);
    update_data.extend_from_slice(&new_info);

    let mut file = fs::File::create(path).unwrap();
    let serialized = serde_json::to_string_pretty(&update_data).expect("Failed to serialize JSON");
    file.write_all(serialized.as_bytes())
        .expect("Failed to write file");
}

pub fn project_read_json_data(path: &str) -> Vec<Project> {
    let mut data_base = File::open(path).unwrap();
    let mut data = String::new();
    data_base.read_to_string(&mut data).unwrap();

    let venue_data: Vec<Project> = serde_json::from_str(&data).expect("Failed to parse JSON");
    venue_data
}
