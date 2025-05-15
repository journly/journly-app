use crate::errors::MyError;
use crate::models::api::users::User;
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

    pub async fn get_user(&self, user_id: Uuid) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields FROM users
            WHERE users.id = $1;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());

        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&user_id])
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

    pub async fn get_user_by_username(&self, username: String) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields FROM users
            WHERE users.username = $1;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&username])
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

    pub async fn get_user_by_email(&self, email: String) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields FROM users
            WHERE users.email = $1;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&email])
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

    pub async fn add_user(
        &self,
        username: String,
        password_hash: String,
        password_salt: Vec<u8>,
    ) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            INSERT INTO users(id, username, password_hash, password_salt)
            VALUES (gen_random_uuid(), $1, $2, $3)
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&username, &password_hash, &password_salt])
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

    pub async fn update_user_display_name(
        &self,
        user_id: Uuid,
        new_display_name: String,
    ) -> Result<Option<String>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            UPDATE users
            SET display_name = $1
            WHERE users.id = $2
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&new_display_name, &user_id])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(user) => Ok(user.display_name),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_user_password_hash(
        &self,
        user_id: Uuid,
        new_password_hash: String,
    ) -> Result<(), MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            UPDATE users
            SET password_hash = $1
            WHERE users.id = $2
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&new_password_hash, &user_id])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(_) => Ok(()),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_user_email(
        &self,
        user_id: Uuid,
        new_email: String,
    ) -> Result<Option<String>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            UPDATE users
            SET email = $1
            WHERE users.id = $2
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&new_email, &user_id])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(user) => Ok(user.email),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&user_id])
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
