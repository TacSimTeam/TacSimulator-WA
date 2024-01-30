use crate::consts::db_url;
use crate::entities::user::UserDto;
use crate::repositories::postgres::PgConn;
use crate::usecases::dmg::create_new_dmg;
use crate::usecases::user::user_validation;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValidationResult {
    Success,
    CreateNewUser,
    PassWordIncorrect,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDmgName {
    pub r#type: ValidationResult,
    pub dmg_name: Option<String>,
}

impl IntoResponse for UserDmgName {
    fn into_response(self) -> Response {
        let status = match self.r#type {
            ValidationResult::Success => StatusCode::OK,
            ValidationResult::CreateNewUser => StatusCode::CREATED,
            ValidationResult::PassWordIncorrect => StatusCode::UNAUTHORIZED,
        };
        let body = Json(self.clone());
        (status, body).into_response()
    }
}

pub async fn load_user_router() -> Router {
    dotenvy::dotenv().ok();
    let db_url = db_url();
    let conn = PgConn::new(db_url).await;
    let shared_db_conn = Arc::new(conn);
    Router::new()
        .route("/dmg_name", post(dmg_name))
        .with_state(shared_db_conn)
}

pub async fn dmg_name(
    State(conn): State<Arc<PgConn>>,
    Json(user): Json<UserDto>,
) -> impl IntoResponse {
    return match user_validation(&conn, user.clone()).await {
        Ok(validation_result) => {
            if validation_result.0 {
                UserDmgName {
                    r#type: ValidationResult::Success,
                    dmg_name: Some(validation_result.1.create_dmg_path()),
                }
            } else {
                UserDmgName {
                    r#type: ValidationResult::PassWordIncorrect,
                    dmg_name: None,
                }
            }
        }
        Err(_) => {
            let user = &conn.save(user.clone()).await.unwrap();

            let created_dmg_path = user.create_dmg_path();
            create_new_dmg(created_dmg_path.clone()).unwrap();
            UserDmgName {
                r#type: ValidationResult::CreateNewUser,
                dmg_name: Some(created_dmg_path),
            }
        }
    };
}
