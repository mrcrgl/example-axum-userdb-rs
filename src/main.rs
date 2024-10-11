mod config;
mod models;
mod persistence;
mod restapi;
mod services;

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract,
    extract::Path,
    http::StatusCode,
    response,
    routing::{get, post},
    Extension, Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use crate::config::{try_read_config, Config};
use crate::restapi::user::create_user_service_router;
use crate::services::user_service::UserService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let config = try_read_config();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();


    let user_service = UserService::new_for_config(config.user_service).await;
    let user_service_router = create_user_service_router(user_service);

    let app = Router::new()
        .nest("/users", user_service_router)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Listen on port: :3000");

    axum::serve(listener, app).await?;

    Ok(())
}
