use crate::entities::user::{User, UserDto};
use crate::error::db_error::DBError;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgConn {
    pub conn: PgPool,
}

impl PgConn {
    pub async fn new(url: String) -> Self {
        PgConn {
            conn: PgPool::connect(&url).await.unwrap(),
        }
    }
    pub async fn save(&self, user: UserDto) -> Result<User, DBError> {
        let sql = r#"
            INSERT INTO
                users (name, password)
            VALUES
                ($1, $2)
            RETURNING
                id, name, password
        "#;
        let mut tx = self.conn.begin().await.unwrap();
        let user = match sqlx::query_as(&sql)
            .bind(&user.name)
            .bind(&user.password)
            .fetch_one(&mut *tx)
            .await
        {
            Ok(user) => user,
            Err(_) => {
                tx.rollback().await.unwrap();
                return Err(DBError::InsertError);
            }
        };
        tx.commit().await.unwrap();
        Ok(user)
    }

    pub async fn delete(&mut self, name: String) -> Result<(), DBError> {
        let sql = r#"
            DELETE FROM
                users
            WHERE
                name=$1
        "#;
        let mut tx = self.conn.begin().await.unwrap();
        if let Err(_) = sqlx::query(&sql).bind(&name).execute(&mut *tx).await {
            tx.rollback().await.unwrap();
            return Err(DBError::NotFound);
        };
        tx.commit().await.unwrap();
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<User>, DBError> {
        let sql = r#"
            SELECT
                id, name, password
            FROM
                users
        "#;
        let result = sqlx::query_as(&sql).fetch_all(&self.conn).await.unwrap();
        Ok(result)
    }
}
