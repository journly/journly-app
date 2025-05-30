use crate::models::user::User;
use crate::schema::{budget_planners, personal_budgets};
use bigdecimal::BigDecimal;
use bon::Builder;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use super::trip::Trip;

#[derive(Clone, Debug, Queryable, Insertable, Associations, Identifiable, Selectable, Builder)]
#[diesel(belongs_to(Trip))]
#[diesel(table_name = budget_planners)]
pub struct BudgetPlanner {
    #[builder(default = Uuid::new_v4())]
    id: Uuid,
    trip_id: Uuid,
    total_budget: Option<BigDecimal>,
    currency: Option<String>,
    accommodation_budget: Option<BigDecimal>,
    transportation_budget: Option<BigDecimal>,
    food_dining_budget: Option<BigDecimal>,
    activities_budget: Option<BigDecimal>,
    shopping_budget: Option<BigDecimal>,
}

impl BudgetPlanner {
    pub async fn get_from_trip(
        conn: &mut AsyncPgConnection,
        trip: &Trip,
    ) -> QueryResult<BudgetPlanner> {
        BudgetPlanner::belonging_to(trip)
            .select(BudgetPlanner::as_select())
            .first(conn)
            .await
    }

    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<BudgetPlanner> {
        diesel::insert_into(budget_planners::table)
            .values(self)
            .get_result(conn)
            .await
    }
}

#[derive(Clone, Debug, Queryable, Insertable, Associations, Identifiable, Selectable, Builder)]
#[diesel(belongs_to(Trip))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = personal_budgets)]
pub struct PersonalBudget {
    #[builder(default = Uuid::new_v4())]
    id: Uuid,
    trip_id: Uuid,
    user_id: Uuid,
    total_budget: Option<BigDecimal>,
    accommodation_budget: Option<BigDecimal>,
    transportation_budget: Option<BigDecimal>,
    food_dining_budget: Option<BigDecimal>,
    activities_budget: Option<BigDecimal>,
    shopping_budget: Option<BigDecimal>,
    currency: Option<String>,
    #[builder(default = false)]
    personal_budget_enabled: bool,
}

impl PersonalBudget {
    pub async fn get_from_trip(
        conn: &mut AsyncPgConnection,
        trip: &Trip,
    ) -> QueryResult<PersonalBudget> {
        PersonalBudget::belonging_to(trip)
            .select(PersonalBudget::as_select())
            .first(conn)
            .await
    }

    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<PersonalBudget> {
        diesel::insert_into(personal_budgets::table)
            .values(self)
            .get_result(conn)
            .await
    }
}

