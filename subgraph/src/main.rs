use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
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
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .enable_federation()
        .enable_subscription_in_federation()
        .finish();

    let app = Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .layer(CorsLayer::permissive());

    println!("GraphiQL IDE: http://localhost:8001");

    axum::serve(TcpListener::bind("127.0.0.1:8001").await.unwrap(), app)
        .await
        .unwrap();
}

use async_graphql::{Object, ID};

pub type SubgraphSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(Clone)]
struct Author {
    id: ID,
}

#[Object]
impl Author {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn name(&self) -> String {
        format!("Author {}", self.id.as_str())
    }
}

pub struct Query;

#[Object]
impl Query {
    #[graphql(entity)]
    async fn author_by_id(&self, id: ID) -> Author {
        Author { id }
    }

    async fn hello(&self) -> &str {
        "hello"
    }
}
