use async_graphql::{EmptySubscription, Schema};
use project_management::{
    config::{owner_data::OwnerData, project_data::ProjectData},
    handler::graphql_handler::{Mutation, Query},
};
use rocket::routes;

use project_management::{graphql_mutation, graphql_playground, graphql_query};

#[rocket::launch]
fn rocket() -> _ {
    let owner_db = OwnerData::init();
    let project_db = ProjectData::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(owner_db)
        .data(project_db)
        .finish();
    rocket::build().manage(schema).mount(
        "/",
        routes![graphql_query, graphql_mutation, graphql_playground],
    )
}
