use sqlx::PgPool;
use crate::model::user::User;
use anyhow::Result;

pub trait UserRepository {
    fn save(conn: &PgPool, user: User) -> Result<User>;
    fn delete(conn: &PgPool, id: i32) -> Result<()>;
    fn list(conn: &PgPool) -> Result<Vec<User>>;
    fn find_by_id(conn: &PgPool, id: i32) -> Result<User>;
}