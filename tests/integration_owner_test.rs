use project_management::{
    config::{json_read_write::{owner_read_json_data, owner_add_json}, owner_data::OwnerData},
    schema::owner_schema::Owner,
};

use rocket::http::Status;

#[test]
fn for_create_owner() {
    let owner = Owner {
        _id: None,
        name: "Akshit".to_string(),
        email: "akshit123@gmal.com".to_string(),
        phone: "8277199288".to_string(),
    };

    let owner_db = OwnerData::init();
    let _check_data = owner_db.create_owner(owner.clone());
    let mut _data = owner_read_json_data("JSON/owner.json");
    let last_index = _data.len() - 1;
    let check_data = _data[last_index].name.clone();
    _data.remove(last_index);

    owner_add_json(_data, "JSON/owner.json");
    println!("last{}", last_index);
    assert_eq!(check_data, owner.name);
}

#[test]
fn test_for_get_owner() {
    let owner = OwnerData::init();
    let get_owner = owner.get_owners();
    let check_data = owner_read_json_data("JSON/owner.json");
    assert_eq!(get_owner, Ok(check_data));
}

#[test]
fn for_get_single_owner() {
    let owner_data = OwnerData::init();
    let _data = owner_data.single_owner(&"859dd8ab-f058-473f-8fb6-75b797265ece".to_string());
    let owner = Owner {
        _id: Some("859dd8ab-f058-473f-8fb6-75b797265ece".to_string()),
        name: "Akshit ".to_string(),
        email: "axitpatel@gmail.com".to_string(),
        phone: "9826541732".to_string(),
    };

    assert_eq!(Ok(owner), (_data));
    let owner = OwnerData::init();
    let _data = owner.single_owner(&"11--1-1--1".to_string());
    // assert_eq!(Status::NotFound, Err(_data));
    assert_eq!(_data, Err(Status::NotFound));
}
