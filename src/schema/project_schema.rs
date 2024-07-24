use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

//project schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(InputObject)]
pub struct CreateProject {
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(InputObject)]
pub struct FetchProject {
    pub _id: String,
}
