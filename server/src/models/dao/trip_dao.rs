use chrono::NaiveDate;
use redis::Commands;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

use super::Data;
use crate::{
    errors::MyError,
    models::{
        api::{trips::UpdateTrip, ToSql},
        schema::Trip,
    },
};

#[derive(Serialize, Deserialize, PostgresMapper, FromRedisValue, ToRedisArgs)]
#[pg_mapper(table = "trip_details")]
pub struct TripDetails {
    id: Uuid,
    owner_id: Uuid,
    title: String,
    trip_image: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

const EXPIRE_TIME_SECONDS: i64 = 10000;

impl Data<Trip> {
    pub async fn get_all_trips(&self) -> Result<Vec<TripDetails>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            SELECT $table_fields FROM trip_details;
            "#;
        let stmt = stmt.replace("$table_fields", &TripDetails::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let trips = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| TripDetails::from_row_ref(row).unwrap())
            .collect::<Vec<TripDetails>>();

        Ok(trips)
    }

    pub async fn get_trip(&self, trip_id: Uuid) -> Result<TripDetails, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("trip_details:{}", trip_id);

        if let Ok(value) = cache.get(&cache_key) {
            let trip: TripDetails = value;

            let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

            return Ok(trip);
        }

        let stmt = r#"
            SELECT $table_fields FROM trip_details
            WHERE trip_details.id = $trip_id;
            "#;
        let stmt = stmt.replace("$table_fields", &TripDetails::sql_table_fields());
        let stmt = stmt.replace("$trip_id", &trip_id.to_string());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| TripDetails::from_row_ref(row).unwrap())
            .collect::<Vec<TripDetails>>()
            .pop();

        match result {
            Some(trip) => Ok(trip),
            _ => Err(MyError::NotFound),
        }
    }

    pub async fn add_trip(&self, creator_user_id: Uuid) -> Result<TripDetails, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let stmt = r#"
            WITH new_dates AS (
                INSERT INTO dates(id)
                VALUES (gen_random_uuid())
                RETURNING *
            ) new_trip AS (
                INSERT INTO trips(id, owner_id, dates_id)
                SELECT gen_random_uuid(), $user_id, id FROM new_dates
                RETURNING *
            )
            SELECT $table_fields 
            FROM new_trip 
            INNER JOIN new_dates
            ON new_trip.dates_id = new_dates.id;
            "#;
        let stmt = stmt.replace("$table_fields", &Trip::sql_table_fields());
        let stmt = stmt.replace("$user_id", &creator_user_id.to_string());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| TripDetails::from_row_ref(row).unwrap())
            .collect::<Vec<TripDetails>>()
            .pop();

        match result {
            Some(trip) => {
                let trip_id = trip.id;

                let cache_key = format!("trip_details:{}", trip_id);

                let _: () = cache.set(&cache_key, &trip).unwrap();

                let _: () = cache.expire(cache_key, EXPIRE_TIME_SECONDS).unwrap();

                Ok(trip)
            }
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_trip_title(&self, trip_id: Uuid, new_title: String) -> Result<String, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        
    } 

    pub async fn delete_trip_by_id(&self, trip_id: Uuid) -> Result<(), MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let mut cache = self.redis_pool.get().map_err(MyError::RedisPoolError)?;

        let cache_key = format!("trip_details:{}", trip_id);

        if let Ok(value) = cache.get(&cache_key) {
            let _: Trip = value;
            let _: () = cache.del(&cache_key).unwrap();
        }

        let stmt = r#"
            DELETE FROM public.trips
            WHERE trips.id = $trip_id
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$trip_id", &trip_id.to_string());
        let stmt = stmt.replace("$table_fields", &Trip::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[])
            .await
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .map(|row| Trip::from_row_ref(row).unwrap())
            .collect::<Vec<Trip>>()
            .pop();

        match result {
            Some(_) => Ok(()),
            _ => Err(MyError::NotFound),
        }
    }
}
