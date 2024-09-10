use axum::{routing::get, Router};
use http::Method;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod routes;
mod services;

#[tokio::main]
async fn main() {
    services::build_client::build_client("./client/".to_string());
    dotenvy::dotenv().expect("Cannot access dot env files");

    let server_url = std::env::var("SERVER_URL").unwrap_or("127.0.0.1:3000".to_owned());
    let db_url = std::env::var("DATABASE_URL").expect("Database Url not found");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db_pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(9))
        .connect(&db_url)
        .await
        .expect("cannot connect to the database");

    let listner = TcpListener::bind(server_url)
        .await
        .expect("Cannot listen to the port");

    println!("Server is running on {}", listner.local_addr().unwrap());

    let serve_dir = ServeDir::new("./client/dist/");
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);
    let app = Router::new()
        .nest_service("/", serve_dir)
        .route(
            "/api/tasks",
            get(routes::tasks::get_tasks).post(routes::tasks::create_task),
        )
        .route(
            "/api/tasks/:id",
            get(routes::tasks::get_task_by_id)
                .patch(routes::tasks::update_task)
                .delete(routes::tasks::delete_task_by_id),
        )
        .layer(TraceLayer::new_for_http())
        .route_layer(cors)
        .with_state(db_pool);

    axum::serve(listner, app)
        .await
        .expect("Cannot start the server");
}
