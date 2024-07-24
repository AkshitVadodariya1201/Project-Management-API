use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

//owner schema
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct CreateOwner {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct FetchOwner {
    pub _id: String,
}
