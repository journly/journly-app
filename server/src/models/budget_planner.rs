use super::trip::Trip;
use crate::{
    models::user::User,
    schema::{budget_planners, personal_budgets},
};
use bigdecimal::BigDecimal;
use bon::Builder;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

#[derive(Clone, Debug, Queryable, Insertable, Associations, Identifiable, Selectable, Builder)]
#[diesel(belongs_to(Trip))]
#[diesel(table_name = budget_planners)]
pub struct BudgetPlanner {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub trip_id: Uuid,
    pub total_budget: Option<BigDecimal>,
    pub currency: Option<String>,
    pub accommodation_budget: Option<BigDecimal>,
    pub transportation_budget: Option<BigDecimal>,
    pub food_dining_budget: Option<BigDecimal>,
    pub activities_budget: Option<BigDecimal>,
    pub shopping_budget: Option<BigDecimal>,
}

impl BudgetPlanner {
    pub async fn get_from_trip(
        conn: &mut AsyncPgConnection,
        trip_id: &Uuid,
    ) -> QueryResult<BudgetPlanner> {
        budget_planners::table
            .filter(budget_planners::trip_id.eq(trip_id))
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
    pub id: Uuid,
    pub trip_id: Uuid,
    pub user_id: Uuid,
    pub total_budget: Option<BigDecimal>,
    pub accommodation_budget: Option<BigDecimal>,
    pub transportation_budget: Option<BigDecimal>,
    pub food_dining_budget: Option<BigDecimal>,
    pub activities_budget: Option<BigDecimal>,
    pub shopping_budget: Option<BigDecimal>,
    pub currency: Option<String>,
    #[builder(default = false)]
    pub personal_budget_enabled: bool,
}

impl PersonalBudget {
    pub async fn get_from_trip(
        conn: &mut AsyncPgConnection,
        trip_id: &Uuid,
        user_id: &Uuid,
    ) -> QueryResult<PersonalBudget> {
        personal_budgets::table
            .filter(personal_budgets::trip_id.eq(trip_id))
            .filter(personal_budgets::user_id.eq(user_id))
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
