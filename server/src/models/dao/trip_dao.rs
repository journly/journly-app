use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;
use uuid::Uuid;

use super::Data;
use crate::{
    errors::MyError,
    models::{
        api::{dates::Dates, ToSql},
        schema::Trip,
    },
};

#[derive(Serialize, Deserialize)]
pub struct TripDetails {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub image_url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

impl TripDetails {
    pub fn sql_table_fields(&self) -> String {
        return format!(" trips.id, owner_id, title, image_url, start_date, end_date ");
    }

    pub fn from_row_ref(row: &Row) -> Result<TripDetails, ()> {
        
        if let Err(e) = row.try_get("id") {
            Err(())
        };

        if let Err(e) = row.try_get("owner_id") {
            Err(())
        };

        Err(())
    }
}

impl Data<Trip> {
    pub async fn get_all_trips(&self) -> Result<Vec<TripDetails>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"SELECT
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

        let stmt = r#"
            SELECT $table_fields FROM trip_details
            WHERE trip_details.id = '$trip_id';
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

        let stmt = r#"
            WITH new_dates AS (
                INSERT INTO dates(id)
                VALUES (gen_random_uuid())
                RETURNING *
            ), new_trip AS (
                INSERT INTO trips(id, owner_id, dates_id)
                SELECT gen_random_uuid(), '$user_id', id FROM new_dates
                RETURNING *
            )
            SELECT $table_fields 
            FROM new_trip 
            INNER JOIN new_dates
            ON new_trip.dates_id = new_dates.id;
            "#;
        let stmt = stmt.replace("$table_fields", &TripDetails::sql_table_fields());
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
            Some(trip) => Ok(trip),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_trip_title(
        &self,
        trip_id: Uuid,
        new_title: String,
    ) -> Result<String, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            UPDATE trips
            SET title = '$new_title'
            WHERE trips.id = '$trip_id'
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$new_title", &new_title);
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
            Some(trip) => Ok(trip.title),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_trip_owner(
        &self,
        trip_id: Uuid,
        new_owner_id: Uuid,
    ) -> Result<Uuid, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            UPDATE trips
            SET trips.owner_id = '$new_owner_id'
            WHERE trips.id = '$trip_id'
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$new_owner_id", &new_owner_id.to_string());
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
            Some(trip) => Ok(trip.owner_id),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_trip_dates(
        &self,
        trip_id: Uuid,
        new_dates: Dates,
    ) -> Result<Dates, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            UPDATE dates
            SET $new_values
            WHERE dates.id in (
                SELECT dates_id 
                FROM trips
                WHERE trips.id = '$trip_id'
            )
            RETURNING $table_fields; 
            "#;

        let stmt = stmt.replace("$new_values", &new_dates.to_sql_values());
        let stmt = stmt.replace("$trip_id", &trip_id.to_string());
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
            Some(new_dates) => Ok(new_dates),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn update_trip_image_url(
        &self,
        trip_id: Uuid,
        new_image_url: Option<String>,
    ) -> Result<Option<String>, MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
           UPDATE trips
           SET trips.image_url = '$new_image_url'
           WHERE trips.id = '$trip_id'
           RETURNING $table_fields;
            "#;

        match new_image_url {
            Some(image_url) => stmt.replace("$new_image_url", &image_url),
            _ => stmt.replace("$new_image_url", "NULL"),
        };

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
            Some(trip) => Ok(trip.image_url),
            _ => Err(MyError::PGError),
        }
    }

    pub async fn delete_trip(&self, trip_id: Uuid) -> Result<(), MyError> {
        let db = self.pg_pool.get().await.map_err(MyError::PGPoolError)?;

        let stmt = r#"
            DELETE FROM trips
            WHERE trips.id = '$trip_id'
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
