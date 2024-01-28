use crate::entities::user::User;
use crate::error::db_error::DBError;
use axum::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> Result<User, DBError>;
    async fn delete(&self, id: i32) -> Result<(), DBError>;
    async fn list(&self) -> Result<Vec<User>, DBError>;
}
