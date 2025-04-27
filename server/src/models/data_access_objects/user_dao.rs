use crate::errors::MyError;
use crate::models::api::users::UpdateUser;
use crate::models::schema::User;
use redis::Commands;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use super::Table;

const EXPIRE_TIME_SECONDS: i64 = 10000;

impl Table<User> {
    pub async fn get_users(&self) -> Result<Vec<User>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields FROM public.users;
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

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("user:{}", user_id);

        if let Ok(value) = cache.get(&cache_key) {
            let user: User = value;

            let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

            return Ok(user);
        }

        let stmt = r#"
            SELECT $table_fields FROM public.users WHERE users.id = $user_id;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = stmt.replace("$user_id", &format!("'{}'", user_id));

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
            Some(user) => {
                let _: () = cache.set(&cache_key, &user).unwrap();

                let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

                Ok(user)
            }
            _ => Err(MyError::NotFound),
        }
    }

    pub async fn add_user(&self, new_user: User) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let stmt = r#"
            INSERT INTO public.users(id, display_name, username, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(
                &stmt,
                &[
                    &new_user.id,
                    &new_user.display_name,
                    &new_user.username,
                    &new_user.password_hash,
                ],
            )
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop();

        match result {
            Some(user) => {
                let cache_key = format!("user:{}", new_user.id);

                let _: () = cache.set(&cache_key, &user).unwrap();

                let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

                Ok(user)
            }
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_user_by_id(
        &self,
        user_id: Uuid,
        update: UpdateUser,
    ) -> Result<User, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;
        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("user:{}", user_id);

        if let Ok(value) = cache.get(&cache_key) {
            let _: User = value; // get rid of compiler warning
            let _: () = cache.del(&cache_key).unwrap();
        }

        let stmt = r#"
            UPDATE public.users 
            SET $new_info WHERE id = $user_id
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = stmt.replace("$user_id", &format!("'{}'", user_id));
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
            Some(user) => {
                let _: () = cache.set(&cache_key, &user).unwrap();

                let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

                Ok(user)
            }
            _ => Err(MyError::PGError),
        }
    }

    pub async fn delete_user_by_id(&self, user_id: Uuid) -> Result<(), MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("user:{}", user_id);

        if let Ok(value) = cache.get(&cache_key) {
            let _: User = value; // to get rid of compiler warning
            let _: () = cache.del(&cache_key).unwrap();
        }
        let stmt = r#"
            DELETE FROM public.users WHERE id = $user_id
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$user_id", &format!("'{}'", user_id));
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
