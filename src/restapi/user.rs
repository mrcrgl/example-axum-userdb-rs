use crate::models::user::User;
use crate::restapi::input::validation::{Validate, ValidationError};
use crate::services::user_service::{AddUserInput, UserService};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{extract, response, Extension, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn create_user_service_router(service: UserService) -> Router {
    let mut router = Router::new()
        .route("/:id", get(get_user).delete(delete_user))
        .route("/", get(list_user).post(add_user))
        .layer(Extension(Arc::new(service)));

    router
}

#[derive(Serialize)]
struct AddUserResponse {
    succeed: bool,
    id: Option<uuid::Uuid>,
    errors: Vec<String>,
}

#[derive(Default, Deserialize)]
struct UserInput {
    username: String,
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

async fn add_user(
    Extension(service): Extension<Arc<UserService>>,
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

    let input = AddUserInput {
        username: user.username.clone(),
        email: user.username,
    };

    let user = service.add(input).await.unwrap();

    (
        StatusCode::OK,
        response::Json(AddUserResponse {
            id: Some(user.id),
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
    Extension(service): Extension<Arc<UserService>>,
) -> response::Json<ListUserResponse> {
    let users = service.list().await.unwrap();

    response::Json(ListUserResponse { users })
}

#[derive(Serialize)]
struct GetUserResponse {
    user: Option<User>,
}

async fn get_user(
    Extension(service): Extension<Arc<UserService>>,
    Path(id): Path<uuid::Uuid>,
) -> response::Json<GetUserResponse> {
    let user = service.get(id).await.unwrap();

    response::Json(GetUserResponse { user })
}

#[derive(Serialize)]
struct DeleteUserResponse {
    succeed: bool,
}

async fn delete_user(
    Extension(service): Extension<Arc<UserService>>,
    Path(id): Path<uuid::Uuid>,
) -> response::Json<DeleteUserResponse> {
    let _ = service.delete(id).await.unwrap();

    response::Json(DeleteUserResponse { succeed: true })
}
