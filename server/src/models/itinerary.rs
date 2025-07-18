use super::{expenses::Expense, location::Location, trip::Trip};
use crate::{
    schema::{expenses, itinerary_items, locations},
    util::errors::{AppError, AppResult},
    views::{EncodableItineraryItem, EncodableLocation, ItineraryExpense},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Trip, foreign_key = trip_id))]
#[diesel(belongs_to(Location, foreign_key = location_id))]
#[diesel(belongs_to(Expense, foreign_key = expense_id))]
pub struct ItineraryItem {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub title: Option<String>,
    pub activity_type: Option<String>,
    pub location_id: Option<Uuid>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub expense_id: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = itinerary_items)]
pub struct NewItineraryItem {
    pub trip_id: Uuid,
    pub title: Option<String>,
    pub activity_type: Option<String>,
    pub location_id: Option<Uuid>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub expense_id: Option<Uuid>,
    pub notes: Option<String>,
}

impl NewItineraryItem {
    pub async fn create_from_encodable(
        conn: &mut AsyncPgConnection,
        item: &EncodableItineraryItem,
        trip_id: &Uuid,
    ) -> QueryResult<ItineraryItem> {
        let location_id = match &item.location {
            Some(location) => match create_location(conn, &location).await {
                Ok(id) => Some(id),
                Err(_) => None,
            },
            None => None,
        };

        let expense_id = match &item.cost {
            Some(cost) => match create_expense(conn, trip_id, &cost).await {
                Ok(id) => Some(id),
                Err(_) => None,
            },
            None => None,
        };

        let new_item = NewItineraryItem {
            trip_id: *trip_id,
            title: item.title.to_owned(),
            activity_type: item.activity_type.to_owned(),
            location_id,
            start_time: item.start_time,
            end_time: item.end_time,
            expense_id,
            notes: item.notes.to_owned(),
        };

        diesel::insert_into(itinerary_items::table)
            .values(new_item)
            .get_result(conn)
            .await
    }

    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<ItineraryItem> {
        diesel::insert_into(itinerary_items::table)
            .values(self)
            .get_result(conn)
            .await
    }
}

async fn create_expense(
    conn: &mut AsyncPgConnection,
    trip_id: &Uuid,
    expense: &ItineraryExpense,
) -> QueryResult<Uuid> {
    let new_id = Uuid::new_v4();
    diesel::insert_into(expenses::table)
        .values((
            expenses::id.eq(new_id),
            expenses::trip_id.eq(trip_id),
            expenses::cost.eq(&expense.cost),
            expenses::currency.eq(&expense.currency),
        ))
        .execute(conn)
        .await?;

    Ok(new_id)
}

async fn create_location(
    conn: &mut AsyncPgConnection,
    location: &EncodableLocation,
) -> QueryResult<Uuid> {
    let new_id = Uuid::new_v4();
    diesel::insert_into(locations::table)
        .values((
            locations::id.eq(new_id),
            locations::address.eq(&location.address),
            locations::display_name.eq(&location.display_name),
            locations::longitude.eq(location.longitude),
            locations::latitude.eq(location.latitude),
        ))
        .execute(conn)
        .await?;

    Ok(new_id)
}

impl EncodableItineraryItem {
    async fn update_location_and_expense(
        &self,
        conn: &mut AsyncPgConnection,
        trip_id: &Uuid,
    ) -> QueryResult<(Option<Uuid>, Option<Uuid>)> {
        // can unwrap because this will only be called after update(..) validates this struct
        let item_id = self.id.unwrap();

        let current_item = itinerary_items::table
            .find(item_id)
            .select(ItineraryItem::as_select())
            .first(conn)
            .await?;

        let location_id = match &self.location {
            Some(new_location) => match update_location(conn, &current_item, &new_location).await {
                Ok(id) => Some(id),
                Err(_) => {
                    eprintln!("Failed to update itinerary item location.");
                    None
                }
            },
            None => None,
        };

        let expense_id = match &self.cost {
            Some(new_expense) => {
                match update_expense(conn, &current_item, &new_expense, trip_id).await {
                    Ok(id) => Some(id),
                    Err(_) => {
                        eprintln!("Failed to update itinerary item cost.");
                        None
                    }
                }
            }
            None => None,
        };

        Ok((location_id, expense_id))
    }

    pub async fn add(&self, conn: &mut AsyncPgConnection, trip_id: &Uuid) -> AppResult<Uuid> {
        let new_item_id = Uuid::new_v4();

        let (location_id, expense_id) = match self.update_location_and_expense(conn, trip_id).await
        {
            Ok(res) => Ok(res),
            Err(_) => Err(AppError::InternalError),
        }?;

        match diesel::insert_into(itinerary_items::table)
            .values((
                itinerary_items::id.eq(new_item_id),
                itinerary_items::activity_type.eq(&self.activity_type),
                itinerary_items::start_time.eq(self.start_time),
                itinerary_items::end_time.eq(self.end_time),
                itinerary_items::location_id.eq(location_id),
                itinerary_items::expense_id.eq(expense_id),
                itinerary_items::notes.eq(&self.notes),
            ))
            .execute(conn)
            .await
        {
            Ok(_) => Ok(new_item_id),
            Err(_) => Err(AppError::InternalError),
        }
    }

    pub async fn update(&self, conn: &mut AsyncPgConnection, trip_id: &Uuid) -> AppResult<()> {
        if self.id.is_none() {
            return Err(AppError::BadRequest("Missing id.".to_string()));
        };

        let item_id = self.id.unwrap();

        let (location_id, expense_id) = match self.update_location_and_expense(conn, trip_id).await
        {
            Ok(res) => Ok(res),
            Err(_) => Err(AppError::InternalError),
        }?;

        match diesel::update(itinerary_items::table)
            .filter(itinerary_items::id.eq(item_id))
            .set((
                itinerary_items::title.eq(&self.title),
                itinerary_items::activity_type.eq(&self.activity_type),
                itinerary_items::start_time.eq(self.start_time),
                itinerary_items::end_time.eq(self.end_time),
                itinerary_items::location_id.eq(location_id),
                itinerary_items::expense_id.eq(expense_id),
                itinerary_items::notes.eq(&self.notes),
            ))
            .execute(conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => {
                eprintln!("Failed to update itinerary item.");
                Err(AppError::InternalError)
            }
        }
    }
}

async fn update_location(
    conn: &mut AsyncPgConnection,
    current_item: &ItineraryItem,
    new_location: &EncodableLocation,
) -> QueryResult<Uuid> {
    if current_item.location_id.is_none() {
        return create_location(conn, &new_location).await;
    }

    let location_id = current_item.location_id.unwrap();

    let previous_location = Location::find(conn, &location_id).await?;

    if previous_location.longitude != new_location.longitude
        || previous_location.latitude != new_location.latitude
    {
        diesel::update(locations::table)
            .filter(locations::id.eq(location_id))
            .set((
                locations::display_name.eq(&new_location.display_name),
                locations::address.eq(&new_location.address),
                locations::longitude.eq(new_location.longitude),
                locations::latitude.eq(new_location.latitude),
            ))
            .execute(conn)
            .await?;
    }

    Ok(location_id)
}

async fn update_expense(
    conn: &mut AsyncPgConnection,
    current_item: &ItineraryItem,
    new_expense: &ItineraryExpense,
    trip_id: &Uuid,
) -> QueryResult<Uuid> {
    if current_item.expense_id.is_none() {
        return create_expense(conn, trip_id, &new_expense).await;
    }

    let expense_id = current_item.expense_id.unwrap();

    let previous_expense = Expense::find(conn, &expense_id).await?;

    if previous_expense.cost != new_expense.cost
        || previous_expense.currency != new_expense.currency
    {
        diesel::update(expenses::table)
            .filter(expenses::id.eq(expense_id))
            .set((
                expenses::cost.eq(&new_expense.cost),
                expenses::currency.eq(&new_expense.currency),
            ))
            .execute(conn)
            .await?;
    }

    Ok(expense_id)
}
