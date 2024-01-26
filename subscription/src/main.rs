use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use books::{Mutation, Query, Storage, Subscription};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, Mutation, Subscription)
        .enable_federation()
        .enable_subscription_in_federation()
        .data(Storage::default())
        .finish();

    let app = Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema))
        .layer(CorsLayer::permissive());

    println!("GraphiQL IDE: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
