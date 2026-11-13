mod routes;
mod internal;
mod models;
mod services;
mod errors;
mod repositories;

use std::env;
use std::time::Duration;
use axum::http::{Method, header};
// use axum::middleware::from_fn_with_state;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use axum::{Router, http};
use axum::routing::{ post };
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

// use crate::routes::{get_handler, health_handler, me_handler, root};
use crate::internal::{create_user_internal_handler, update_user_internal_handler};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth_client: Client,
}

#[tokio::main]
async fn main() {
    let postgres_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let postgres_pool: PgPool = PgPoolOptions::new()
        .max_connections(15)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&postgres_url)
        .await
        .expect("Failed to create Postgres pool");

    let auth_client = Client::builder()
        .cookie_store(false)
        .build()
        .expect("Failed to create reqwest client");

    let app_state = AppState {
        db: postgres_pool,
        auth_client: auth_client,
    };

    sqlx::migrate!("./migrations")
        .run(&app_state.db)
        .await
        .expect("Failed to run migrations");

    let frontend_origin = "http://localhost:7004"
        .parse::<http::HeaderValue>()
        .unwrap();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(frontend_origin))
        // .allow_origin(AllowOrigin::any())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);


    let public = Router::new();
    //     .route("/", get(root))
    //     .route("/health", get(health_handler))
    //     .route("/get", get(get_handler))
    //     .route("/settings", get(settings_get_handler).post(settings_post_handler))
    //     .route("/me", get(me_handler))
    //     .layer(from_fn_with_state(app_state.clone(), user_middleware))
    //     .layer(cors)
    //     .with_state(app_state);

    let internal = Router::new()
        .route("/internal/create_user", post(create_user_internal_handler))
        .route("/internal/update_user", post(update_user_internal_handler))
        .layer(cors)
        .with_state(app_state);


    let public_listener: TcpListener = TcpListener::bind("0.0.0.0:7008").await.unwrap();
    let internal_listener: TcpListener = TcpListener::bind("0.0.0.0:8008").await.unwrap();

    println!("User service running on http://127.0.0.1:7008");
    println!("Internal user service running on http://127.0.0.1:8008");

    let public_server = axum::serve(public_listener, public.into_make_service());
    let internal_server = axum::serve(internal_listener, internal.into_make_service());

    let (public_result, internal_result) = tokio::join!(public_server, internal_server);
    public_result.unwrap();
    internal_result.unwrap();
}