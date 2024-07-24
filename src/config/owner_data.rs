use super::json_read_write::*;
use crate::schema::owner_schema::Owner;
use async_graphql::parser::Error;
use rocket::http::Status;
use uuid::Uuid;

pub struct OwnerData {
    data: Vec<Owner>,
}

impl OwnerData {
    pub fn init() -> Self {
        OwnerData { data: Vec::new() }
    }
    //Owners logic
    pub fn create_owner(&self, new_owner: Owner) -> Result<Owner, Error> {
        let uuid_id = Uuid::new_v4();
        let mut add_new_owner = Vec::new();
        let new_owner = Owner {
            _id: Some(uuid_id.to_string()),
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };
        add_new_owner.push(new_owner.clone());
        owner_add_json(add_new_owner, "JSON/owner.json");
        Ok(new_owner)
    }

    pub fn get_owners(&self) -> Result<Vec<Owner>, Error> {
        let owners = owner_read_json_data("JSON/owner.json");
        Ok(owners)
    }

    pub fn single_owner(&self, id: &String) -> Result<Owner, Status> {
        let mut owners = owner_read_json_data("JSON/owner.json");
        if let Some(owner_id) = owners
            .iter_mut()
            .find(|find_id| find_id._id == Some(id.to_string()))
        {
            return Ok(owner_id.clone());
        }

        Err(Status::NotFound)
    }
}
