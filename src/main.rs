mod models;
mod persistence;
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

fn main() {
    unimplemented!()
}

/*

type UserDatabase = HashMap<uuid::Uuid, User>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let user_db_state = Arc::new(Mutex::new(UserDatabase::default()));

    let app = Router::new()
        .route("/users", post(add_user))
        .route("/users/:id", get(get_user))
        .route("/users", get(list_user))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(user_db_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Listen on port: :3000");

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Default, Deserialize)]
struct UserInput {
    username: String,
}

struct ValidationError {
    message: String,
}

trait Validate {
    fn validate(&self) -> Option<Vec<ValidationError>>;
}

impl Validate for UserInput {
    fn validate(&self) -> Option<Vec<ValidationError>> {
        let mut errors = vec![];
        if self.username.len() < 5 {
            errors.push(ValidationError {
                message: String::from("Username must be longer than 5 chars"),
            });
        }

        if errors.is_empty() {
            return None;
        }
        Some(errors)
    }
}

#[derive(Serialize)]
struct AddUserResponse {
    succeed: bool,
    id: Option<uuid::Uuid>,
    errors: Vec<String>,
}

async fn add_user(
    Extension(user_db): Extension<Arc<Mutex<UserDatabase>>>,
    extract::Json(user): extract::Json<UserInput>,
) -> (StatusCode, response::Json<AddUserResponse>) {
    if let Some(errors) = user.validate() {
        let response_body = AddUserResponse {
            succeed: false,
            id: None,
            errors: errors
                .into_iter()
                .map(|err| err.message)
                .collect::<Vec<_>>(),
        };
        return (StatusCode::BAD_REQUEST, response::Json(response_body));
    }

    let id = uuid::Uuid::new_v4();

    let user = User {
        id,
        username: user.username.clone(),
        email: user.username,
        created_at: chrono::Utc::now(),
    };

    let mut db = user_db.lock().await;
    db.insert(id, user);

    (
        StatusCode::OK,
        response::Json(AddUserResponse {
            id: Some(id),
            succeed: true,
            errors: vec![],
        }),
    )
}

#[derive(Serialize)]
struct ListUserResponse {
    users: Vec<User>,
}

async fn list_user(
    Extension(user_db): Extension<Arc<Mutex<UserDatabase>>>,
) -> response::Json<ListUserResponse> {
    let db = user_db.lock().await;

    let users = db.iter().map(|(_, user)| user.to_owned()).collect();

    response::Json(ListUserResponse { users })
}

#[derive(Serialize)]
struct GetUserResponse {
    user: Option<User>,
}

async fn get_user(
    Extension(user_db): Extension<Arc<Mutex<UserDatabase>>>,
    Path(id): Path<uuid::Uuid>,
) -> response::Json<GetUserResponse> {
    let db = user_db.lock().await;

    let user = db.get(&id).cloned();

    response::Json(GetUserResponse { user })
}
*/
