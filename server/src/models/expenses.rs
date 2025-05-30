use crate::schema::{expense_payers, expenses};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(BudgetPlanner))]
pub struct Expense {
    id: Uuid,
    budget_planner_id: Uuid,
    title: Option<String>,
    cost: Option<BigDecimal>,
    currency: Option<String>,
}

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(primary_key(expense_id, user_id))]
#[diesel(belongs_to(Expense, foreign_key = expense_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct ExpensePayer {
    pub expense_id: Uuid,
    pub user_id: Uuid,
}
