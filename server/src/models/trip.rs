use crate::models::user_trip::UserTrip;
use crate::schema::{budget_planners, itinerary_items, trips, user_trip, users};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
use uuid::Uuid;

use super::expenses::{Expense, ExpensePayer};
use super::itinerary::ItineraryItem;
use super::{
    budget_planner::BudgetPlanner,
    user::{Collaborator, User},
};

#[derive(Clone, Debug, Queryable, Selectable, Identifiable)]
pub struct Trip {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: Option<String>,
    pub banner_image: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub no_collaborators: i32,
}

pub struct TripData(
    Trip,
    BudgetPlanner,
    Vec<(Expense, Vec<ExpensePayer>)>,
    Vec<ItineraryItem>,
);

impl Trip {
    pub async fn get_all(conn: &mut AsyncPgConnection) -> QueryResult<Vec<Trip>> {
        trips::table
            .select(Trip::as_select())
            .get_results(conn)
            .await
    }

    pub async fn find(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<Trip> {
        trips::table.find(id).first(conn).await
    }

    pub async fn get_trip_details(
        conn: &mut AsyncPgConnection,
        trip_id: Uuid,
        user_id: Uuid,
    ) -> QueryResult<TripData> {
        let res: TripData = conn
            .transaction::<TripData, diesel::result::Error, _>(|conn| {
                async move {
                    let trip: Trip = trips::table.find(trip_id).get_result(conn).await?;

                    let trip_budget_planner: BudgetPlanner = budget_planners::table
                        .filter(budget_planners::trip_id.eq(trip_id))
                        .select(BudgetPlanner::as_select())
                        .get_result(conn)
                        .await?;

                    let trip_itinerary_items: Vec<ItineraryItem> = itinerary_items::table
                        .filter(itinerary_items::trip_id.eq(trip_id))
                        .select(ItineraryItem::as_select())
                        .get_results(conn)
                        .await?;

                    Ok(TripData(trip, trip_budget_planner, trip_itinerary_items))
                }
                .scope_boxed()
            })
            .await?;

        Ok(res)
    }

    pub async fn check_collaborator(
        conn: &mut AsyncPgConnection,
        trip_id: Uuid,
        user_id: Uuid,
    ) -> bool {
        match user_trip::table
            .find((user_id, trip_id))
            .select(UserTrip::as_select())
            .get_result(conn)
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn get_collaborators(
        conn: &mut AsyncPgConnection,
        trip_id: Uuid,
    ) -> QueryResult<Vec<Collaborator>> {
        let collaborator: Vec<(User, UserTrip)> = user_trip::table
            .inner_join(users::table)
            .filter(user_trip::trip_id.eq(trip_id))
            .select((User::as_select(), UserTrip::as_select()))
            .load(conn)
            .await?;

        Ok(collaborator
            .iter()
            .map(|tuple| {
                let user = &tuple.0;
                let mapping = &tuple.1;

                Collaborator {
                    id: user.id,
                    username: user.username.clone(),
                    avatar: user.avatar.clone(),
                    permission: mapping.permission.clone(),
                }
            })
            .collect::<Vec<Collaborator>>())
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = trips)]
pub struct NewTrip<'a> {
    pub owner_id: &'a Uuid,
    pub title: Option<&'a str>,
    pub start_date: Option<&'a NaiveDate>,
    pub end_date: Option<&'a NaiveDate>,
}

impl NewTrip<'_> {
    pub async fn create(&self, conn: &mut AsyncPgConnection) -> QueryResult<Trip> {
        conn.transaction(|conn| {
            async move {
                let trip: Trip = diesel::insert_into(trips::table)
                    .values(self)
                    .get_result(conn)
                    .await?;

                BudgetPlanner::builder()
                    .trip_id(trip.id)
                    .build()
                    .insert(conn)
                    .await?;

                Itinerary::builder()
                    .trip_id(trip.id)
                    .build()
                    .insert(conn)
                    .await?;

                Ok(trip)
            }
            .scope_boxed()
        })
        .await
    }
}
