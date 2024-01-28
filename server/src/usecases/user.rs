use crate::entities::user::{User, UserDto};
use crate::error::db_error::DBError;
use crate::repositories::postgres::PgConn;

pub async fn user_validation(conn: &PgConn, user: UserDto) -> Result<(bool, User), DBError> {
    let users = conn.list().await?;
    let found_user = match users.iter().find(|u| u.name == user.name) {
        Some(user) => user,
        None => return Err(DBError::NotFound),
    };
    Ok((found_user.password == user.password, found_user.clone()))
}
