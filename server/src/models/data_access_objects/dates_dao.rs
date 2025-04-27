use redis::Commands;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use super::Table;
use crate::{
    errors::MyError,
    models::{
        api::{dates::UpdateDates, ToSql},
        schema::Dates,
    },
};

const EXPIRE_TIME_SECONDS: i64 = 10000;

impl Table<Dates> {
    pub async fn get_dates(&self) -> Result<Vec<Dates>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields from public.dates; 
            "#;
        let stmt = stmt.replace("$table_fields", &Dates::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let dates = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| Dates::from_row_ref(row).unwrap())
            .collect::<Vec<Dates>>();

        Ok(dates)
    }

    pub async fn get_date_by_id(&self, dates_id: Uuid) -> Result<Dates, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("dates:{}", dates_id);

        if let Ok(value) = cache.get(&cache_key) {
            let date: Dates = value;

            let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

            return Ok(date);
        }

        let stmt = r#"
            SELECT $table_fields from public.dates
            WHERE dates.id = $dates_id;
            "#;
        let stmt = stmt.replace("$table_fields", &Dates::sql_table_fields());
        let stmt = stmt.replace("$date_id", &dates_id.to_string());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| Dates::from_row_ref(row).unwrap())
            .collect::<Vec<Dates>>()
            .pop();

        match result {
            Some(dates) => Ok(dates),
            _ => Err(MyError::NotFound),
        }
    }

    pub async fn add_dates(&self, new_dates: Dates) -> Result<Dates, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("dates:{}", new_dates.id);

        let stmt = r#"
            INSERT INTO public.dates(id, start_date, end_date)
            VALUES ($1, $2, $3)
            RETURNING $table_fields
            "#;
        let stmt = stmt.replace("$table_fields", &Dates::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(
                &stmt,
                &[&new_dates.id, &new_dates.start_date, &new_dates.end_date],
            )
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| Dates::from_row_ref(row).unwrap())
            .collect::<Vec<Dates>>()
            .pop();

        match result {
            Some(date) => {
                let _: () = cache.set(&cache_key, &date).unwrap();

                let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

                Ok(date)
            }
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_date_by_id(
        &self,
        dates_id: Uuid,
        updates: UpdateDates,
    ) -> Result<Dates, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("dates:{}", dates_id);

        if let Ok(value) = cache.get(&cache_key) {
            let _: Dates = value;
            let _: () = cache.del(&cache_key).unwrap();
        }

        let stmt = r#"
            UPDATE public.dates
            SET $new_info 
            WHERE dates.id = $dates_id
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$new_info", &updates.to_sql_values());
        let stmt = stmt.replace("$date_id", &dates_id.to_string());
        let stmt = stmt.replace("$table_fields", &Dates::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| Dates::from_row_ref(row).unwrap())
            .collect::<Vec<Dates>>()
            .pop();

        match result {
            Some(date) => {
                let _: () = cache.set(&cache_key, &date).unwrap();

                let _: () = cache.expire(&cache_key, EXPIRE_TIME_SECONDS).unwrap();

                Ok(date)
            }
            _ => Err(MyError::PGError),
        }
    }

    pub async fn delete_date_by_id(&self, dates_id: Uuid) -> Result<(), MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("dates:{}", dates_id);

        if let Ok(value) = cache.get(&cache_key) {
            let _: Dates = value;
            let _: () = cache.del(&cache_key).unwrap();
        }

        let stmt = r#"
            DELETE FROM public.dates
            WHERE dates.id = $dates_id
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$date_id", &dates_id.to_string());
        let stmt = stmt.replace("$table_fields", &Dates::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| Dates::from_row_ref(row).unwrap())
            .collect::<Vec<Dates>>()
            .pop();

        match result {
            Some(_) => Ok(()),
            _ => Err(MyError::NotFound),
        }
    }
}
