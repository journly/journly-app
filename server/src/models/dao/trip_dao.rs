use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use super::Data;
use crate::{
    errors::MyError,
    models::api::{
        dates::Dates,
        trips::{Trip, TripDetails},
        ToSql,
    },
};

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

        let stmt = r#"
            SELECT $table_fields FROM trip_details
            WHERE trip_details.id = $1;
            "#;
        let stmt = stmt.replace("$table_fields", &TripDetails::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&trip_id])
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
                SELECT gen_random_uuid(), $1, id FROM new_dates
                RETURNING *
            )
            SELECT new_trip.id as id, owner_id, title, image_url, start_date, end_date
            FROM new_trip
            INNER JOIN new_dates
            ON new_trip.dates_id = new_dates.id;
            "#;
        let stmt = db.prepare(stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&creator_user_id])
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
            SET title = $1
            WHERE trips.id = $2
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &Trip::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&new_title, &trip_id])
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
            SET owner_id = $1
            WHERE trips.id = $2
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &Trip::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&new_owner_id, &trip_id])
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
                WHERE trips.id = $1
            )
            RETURNING $table_fields;
            "#;

        let stmt = stmt.replace("$new_values", &new_dates.to_sql_values());
        let stmt = stmt.replace("$table_fields", &Dates::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&trip_id])
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
           SET image_url = $1
           WHERE trips.id = $2
           RETURNING $table_fields;
            "#;

        let stmt = stmt.replace("$table_fields", &Trip::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&new_image_url, &trip_id])
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
            WHERE trips.id = $1
            RETURNING $table_fields;
            "#;
        let stmt = stmt.replace("$table_fields", &Trip::sql_table_fields());
        let stmt = db.prepare(&stmt).await.unwrap();

        let result = db
            .query(&stmt, &[&trip_id.to_string()])
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
