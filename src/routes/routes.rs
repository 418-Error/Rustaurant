use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{response::{Html, IntoResponse}, extract::Extension};


pub type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;



pub async fn graphql_playground() -> impl IntoResponse{
    Html(
        playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
        )
    )
}

pub async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello, World!"
    }
}   