use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DiskImage {
    path: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}