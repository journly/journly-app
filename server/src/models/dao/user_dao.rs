use crate::errors::MyError;
use crate::models::api::users::UpdateUser;
use crate::models::api::ToSql;
use crate::models::schema::User;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use super::Data;

impl Data<User> {
    pub async fn get_users(&self) -> Result<Vec<User>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields FROM users;
            "#;

        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let users = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>();

        Ok(users)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields FROM users WHERE users.id = $user_id;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = stmt.replace("$user_id", &user_id.to_string());

        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(user) => Ok(user),
            _ => Err(MyError::NotFound),
        }
    }

    pub async fn add_user(&self, username: String, password_hash: String) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            INSERT INTO users(id, username, password_hash)
            VALUES (gen_random_uuid(), '$username', '$password_hash')
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = stmt.replace("$username", &username);
        let stmt = stmt.replace("$password_hash", &password_hash);
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(user) => Ok(user),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_user_by_id(
        &self,
        user_id: Uuid,
        update: UpdateUser,
    ) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            UPDATE users 
            SET $new_info WHERE id = '$user_id'
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = stmt.replace("$user_id", &user_id.to_string());
        let stmt = stmt.replace("$new_info", &update.to_sql_values());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(user) => Ok(user),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn delete_user_by_id(&self, user_id: Uuid) -> Result<(), MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            DELETE FROM users WHERE id = '$user_id'
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$user_id", &user_id.to_string());
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(_) => Ok(()),
            _ => Err(MyError::NotFound),
        }
    }
}
