use super::{trip::Trip, user::User};
use crate::schema::{expense_payers, expenses, users};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

#[derive(Clone, Debug, Queryable, Associations, Identifiable, Selectable)]
#[diesel(belongs_to(Trip))]
pub struct Expense {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub title: Option<String>,
    pub cost: Option<BigDecimal>,
    pub currency: Option<String>,
}

impl Expense {
    pub async fn get_expenses_with_payers(
        conn: &mut AsyncPgConnection,
        trip_id: &Uuid,
    ) -> QueryResult<Vec<(Expense, Vec<User>)>> {
        let expense_list: Vec<Expense> = expenses::table
            .filter(expenses::trip_id.eq(trip_id))
            .select(Expense::as_select())
            .load(conn)
            .await?;

        let expense_payer_list: Vec<(ExpensePayer, User)> =
            ExpensePayer::belonging_to(&expense_list)
                .inner_join(users::table)
                .select((ExpensePayer::as_select(), User::as_select()))
                .load(conn)
                .await?;

        Ok(expense_payer_list
            .grouped_by(&expense_list)
            .into_iter()
            .zip(expense_list)
            .map(|(expense_payer_tuple, expense)| {
                (
                    expense,
                    expense_payer_tuple
                        .into_iter()
                        .map(|(_, user)| user)
                        .collect(),
                )
            })
            .collect::<Vec<(Expense, Vec<User>)>>())
    }
}

#[derive(Debug, Queryable, Associations, Identifiable, Selectable)]
#[diesel(primary_key(expense_id, user_id))]
#[diesel(belongs_to(Expense, foreign_key = expense_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct ExpensePayer {
    pub expense_id: Uuid,
    pub user_id: Uuid,
}
