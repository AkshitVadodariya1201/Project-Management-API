use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};

use handler::graphql_handler::ProjectSchema;
use rocket::{response::content, routes, State};

pub mod config;
pub mod handler;
pub mod schema;

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<ProjectSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_mutation(
    schema: &State<ProjectSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::get("/")]
pub async fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/echo?stream")]
fn echo_stream(ws: ws::WebSocket) -> ws::Stream!['static] {
    ws::Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}