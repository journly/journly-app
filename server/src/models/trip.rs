use crate::models::user_trip::UserTrip;
use crate::schema::trips::owner_id;
use crate::schema::{documents, expenses, itinerary_items, locations, trips, user_trip, users};
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
use uuid::Uuid;

use super::{
    budget_planner::{BudgetPlanner, PersonalBudget},
    documents::Document,
    expenses::Expense,
    itinerary::ItineraryItem,
    location::Location,
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
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct TripData {
    pub trip: Trip,
    pub collaborators: Vec<Collaborator>,
    pub budget_plan: BudgetPlanner,
    pub personal_budget_plan: PersonalBudget,
    pub trip_expenses: Vec<(Expense, Vec<User>)>,
    pub itinerary_items: Vec<(ItineraryItem, Option<Location>, Option<Expense>)>,
    pub documents: Vec<Document>,
}

impl Trip {
    pub async fn get_all(conn: &mut AsyncPgConnection) -> QueryResult<Vec<Trip>> {
        trips::table
            .select(Trip::as_select())
            .get_results(conn)
            .await
    }

    pub async fn find(conn: &mut AsyncPgConnection, id: &Uuid) -> QueryResult<Trip> {
        trips::table.find(id).first(conn).await
    }

    pub async fn get_trip_data(
        conn: &mut AsyncPgConnection,
        trip_id: &Uuid,
        user_id: &Uuid,
    ) -> QueryResult<TripData> {
        let trip = Self::find(conn, trip_id).await?;
        let collaborators = Self::get_collaborators(conn, trip_id).await?;
        let budget_plan = BudgetPlanner::get_from_trip(conn, trip_id).await?;
        let personal_budget_plan = PersonalBudget::get_from_trip(conn, trip_id, user_id).await?;
        let trip_expenses = Expense::get_expenses_with_payers(conn, trip_id).await?;
        let itinerary_items = Self::get_itinerary(conn, trip_id).await?;
        let documents = Self::get_documents(conn, trip_id).await?;

        Ok(TripData {
            trip,
            collaborators,
            budget_plan,
            personal_budget_plan,
            trip_expenses,
            itinerary_items,
            documents,
        })
    }

    pub async fn check_collaborator(
        conn: &mut AsyncPgConnection,
        trip_id: &Uuid,
        user_id: &Uuid,
    ) -> bool {
        user_trip::table
            .find((user_id, trip_id))
            .select(UserTrip::as_select())
            .first(conn)
            .await
            .is_ok()
    }

    pub async fn get_collaborators(
        conn: &mut AsyncPgConnection,
        id: &Uuid,
    ) -> QueryResult<Vec<Collaborator>> {
        let collaborator: Vec<(User, UserTrip)> = user_trip::table
            .inner_join(users::table)
            .filter(user_trip::trip_id.eq(id))
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

    pub async fn get_itinerary(
        conn: &mut AsyncPgConnection,
        id: &Uuid,
    ) -> QueryResult<Vec<(ItineraryItem, Option<Location>, Option<Expense>)>> {
        itinerary_items::table
            .filter(itinerary_items::trip_id.eq(id))
            .left_join(locations::table)
            .left_join(expenses::table)
            .select((
                ItineraryItem::as_select(),
                Option::<Location>::as_select(),
                Option::<Expense>::as_select(),
            ))
            .load(conn)
            .await
    }

    pub async fn get_documents(
        conn: &mut AsyncPgConnection,
        id: &Uuid,
    ) -> QueryResult<Vec<Document>> {
        documents::table
            .filter(documents::trip_id.eq(id))
            .select(Document::as_select())
            .load(conn)
            .await
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
        conn.transaction(|conn: &mut AsyncPgConnection| {
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

                Ok(trip)
            }
            .scope_boxed()
        })
        .await
    }
}
