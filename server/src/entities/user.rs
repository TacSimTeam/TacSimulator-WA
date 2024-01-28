use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn create_dmg_path(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct UserDto {
    pub name: String,
    pub password: String,
}

impl UserDto {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn password(&self) -> String {
        self.name.clone()
    }
}
