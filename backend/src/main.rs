use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
mod resolvers;
mod cors;
mod db;

use resolvers::Query;
use cors::cors_layer;
use db::init_postgres;

use axum::{
    Router,
    response::{self, IntoResponse},
    routing::{get, post_service},
};
use tokio::net::TcpListener;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    let pool = init_postgres()
        .await
        .expect("Failed to initialize database connection");
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(pool)
        .finish();

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post_service(GraphQL::new(schema.clone())))
        .route("/graphiql", get(graphiql))
        .layer(cors_layer());

    println!("GraphQL endpoint: http://localhost:8000/graphql");
    println!("GraphiQL IDE (root): http://localhost:8000/");

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app)
        .await
        .unwrap();
}
