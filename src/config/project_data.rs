use super::json_read_write::*;
use crate::schema::project_schema::Project;
use async_graphql::parser::Error;
use rocket::http::Status;
use uuid::Uuid;

pub struct ProjectData {
    data: Vec<Project>,
}

impl ProjectData {
    pub fn init() -> Self {
        ProjectData { data: Vec::new() }
    }
    //project logics
    pub fn create_project(&self, new_project: Project) -> Result<Project, Error> {
        let mut add_new_project = Vec::new();
        let uuid_id = Uuid::new_v4();
        let new_project = Project {
            _id: Some(uuid_id.to_string()),
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };
        add_new_project.push(new_project.clone());
        project_add_json(add_new_project, "JSON/project.json");
        Ok(new_project)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let projects = project_read_json_data("JSON/project.json");
        Ok(projects)
    }

    pub fn single_project(&self, id: &String) -> Result<Project, Status> {
        let mut projects = project_read_json_data("JSON/project.json");
        if let Some(project_detail) = projects
            .iter_mut()
            .find(|find_id| find_id._id == Some(id.to_string()))
        {
            return Ok(project_detail.clone());
        }

        Err(Status::NotFound)
    }
}
