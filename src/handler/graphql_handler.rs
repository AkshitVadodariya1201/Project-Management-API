use crate::{
    config::{owner_data::OwnerData, project_data::ProjectData},
    schema::{
        owner_schema::{CreateOwner, FetchOwner, Owner},
        project_schema::{CreateProject, FetchProject, Project},
    },
};
use async_graphql::Subscription;
use async_graphql::{Context, EmptySubscription, Error, FieldResult, Object, Schema};
use redis::Client;
use rocket::futures::Stream;

pub struct Query;

#[Object(extends)]
impl Query {
    //owners query
    async fn owner(&self, ctx: &Context<'_>, input: FetchOwner) -> FieldResult<Owner> {
        let db = &ctx.data_unchecked::<OwnerData>();
        let owner = db.single_owner(&input._id).unwrap();
        Ok(owner)
    }

    async fn get_owners(&self, ctx: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let db = &ctx.data_unchecked::<OwnerData>();
        let owners = db.get_owners().unwrap();
        Ok(owners)
    }

    //projects query
    async fn project(&self, ctx: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let db = &ctx.data_unchecked::<ProjectData>();
        let project = db.single_project(&input._id).unwrap();
        Ok(project)
    }

    async fn get_projects(&self, ctx: &Context<'_>) -> FieldResult<Vec<Project>> {
        let db = &ctx.data_unchecked::<ProjectData>();
        let projects = db.get_projects().unwrap();
        Ok(projects)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    //owner mutation
    async fn create_owner(&self, ctx: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
        let db = &ctx.data_unchecked::<OwnerData>();
        let new_owner = Owner {
            _id: None,
            email: input.email,
            name: input.name,
            phone: input.phone,
        };
        let owner = db.create_owner(new_owner).unwrap();
        Ok(owner)
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProject,
    ) -> FieldResult<Project> {
        let db = &ctx.data_unchecked::<ProjectData>();
        let new_project = Project {
            _id: None,
            owner_id: input.owner_id,
            name: input.name,
            description: input.description,
            status: input.status,
        };
        let project = db.create_project(new_project).unwrap();
        Ok(project)
    }
}

// pub struct SubscriptionRoot;

// #[Subscription]
// impl SubscriptionRoot {
//     async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>, Error> {
//         let client = ctx.data_unchecked::<Client>();
//         let mut conn = client.get_connection()?.as_pubsub();
//         conn.subscribe("values")?;
//         Ok(conn
//             .get_message()
//         )
//     }
// }

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
