mod routes;
mod models;
mod services;
mod repositories;
mod util;
mod errors;
mod middleware;

use std::env;
use std::time::Duration;
use axum::http::{Method, header};
use axum::middleware::from_fn_with_state;
use deadpool_redis::{Config, Pool as RedisPool, Runtime};
use sqlx::postgres::PgPoolOptions;
use axum::{ Router, http };
use axum::routing::{ get, post };
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};


use crate::middleware::auth_middleware;
use crate::routes::{ health_handler, login_handler, logout_handler, me_handler, refresh_handler, register_handler, root };

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cache: RedisPool,
}

#[tokio::main]
async fn main() {
    let redis_url: String = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_pool = Config::from_url(redis_url)
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");
    
    let postgres_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let postgres_pool: PgPool = PgPoolOptions::new()
        .max_connections(15)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&postgres_url)
        .await
        .expect("Failed to create Postgres pool");

    let app_state = AppState {
        db: postgres_pool,
        cache: redis_pool,
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


    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/refresh", post(refresh_handler))
        .route("/me", get(me_handler))
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(cors)
        .with_state(app_state);


    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:7005").await.unwrap();
    println!("Auth service running on http://127.0.0.1:7005");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}