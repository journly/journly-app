use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{
    budget_planner::{BudgetPlanner, PersonalBudget},
    documents::Document,
    expenses::Expense,
    itinerary::ItineraryItem,
    location::Location,
    trip::{Trip, TripData},
    user::{Collaborator, User},
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableUser {
    pub id: Uuid,
    #[schema(example = "journlyuser223")]
    pub username: String,
    #[schema(example = "funemail@journly.com")]
    pub email: String,
    pub avatar: Option<String>,
}

impl From<User> for EncodableUser {
    fn from(value: User) -> Self {
        EncodableUser {
            id: value.id,
            username: value.username,
            email: value.email,
            avatar: value.avatar,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableCollaborator {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub permission: Option<String>,
}

impl From<&Collaborator> for EncodableCollaborator {
    fn from(value: &Collaborator) -> Self {
        Self {
            id: value.id,
            username: value.username.clone(),
            avatar: value.avatar.clone(),
            permission: value.permission.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableTripOverview {
    pub id: Uuid,
    #[schema(example = "Japan Trip 2025")]
    pub title: Option<String>,
    pub banner_image: Option<String>,
    #[schema(example = "2025-12-20")]
    pub start_date: Option<NaiveDate>,
    #[schema(example = "2025-12-20")]
    pub end_date: Option<NaiveDate>,
    pub no_collaborators: i32,
}

impl From<Trip> for EncodableTripOverview {
    fn from(value: Trip) -> Self {
        EncodableTripOverview {
            id: value.id,
            title: value.title,
            banner_image: value.banner_image,
            start_date: value.start_date,
            end_date: value.end_date,
            no_collaborators: value.no_collaborators,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableUserPreview {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
}

impl From<User> for EncodableUserPreview {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            avatar: value.avatar,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableExpense {
    pub id: Uuid,
    pub title: Option<String>,
    #[schema(value_type = String, example = "123.45")]
    pub cost: Option<BigDecimal>,
    #[schema(example = "USD")]
    pub currency: Option<String>,
    pub payers: Vec<EncodableUserPreview>,
}

impl From<&(Expense, Vec<User>)> for EncodableExpense {
    fn from(value: &(Expense, Vec<User>)) -> Self {
        let expense = value.0.clone();
        let payers = &value.1;

        Self {
            id: expense.id,
            title: expense.title,
            cost: expense.cost,
            currency: expense.currency,
            payers: payers
                .iter()
                .map(|p| EncodableUserPreview::from(p.clone()))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableBudgetPlan {
    pub personal_budget: EncodablePersonalBudget,
    pub group_budget: EncodableGroupBudget,
    pub expenses: Vec<EncodableExpense>,
    pub use_personal_budget: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodablePersonalBudget {
    pub currency: Option<String>,
    #[schema(value_type = String, example = "123.45")]
    pub total_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub accommodation_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub transportation_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub food_dining_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub activities_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub shopping_budget: Option<BigDecimal>,
}

impl From<PersonalBudget> for EncodablePersonalBudget {
    fn from(value: PersonalBudget) -> Self {
        Self {
            currency: value.currency,
            total_budget: value.total_budget,
            accommodation_budget: value.accommodation_budget,
            transportation_budget: value.transportation_budget,
            food_dining_budget: value.food_dining_budget,
            activities_budget: value.activities_budget,
            shopping_budget: value.shopping_budget,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableGroupBudget {
    pub currency: Option<String>,
    #[schema(value_type = String, example = "123.45")]
    pub total_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub accommodation_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub transportation_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub food_dining_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub activities_budget: Option<BigDecimal>,
    #[schema(value_type = String, example = "123.45")]
    pub shopping_budget: Option<BigDecimal>,
}

impl From<BudgetPlanner> for EncodableGroupBudget {
    fn from(value: BudgetPlanner) -> Self {
        Self {
            currency: value.currency,
            total_budget: value.total_budget,
            accommodation_budget: value.accommodation_budget,
            transportation_budget: value.transportation_budget,
            food_dining_budget: value.food_dining_budget,
            activities_budget: value.activities_budget,
            shopping_budget: value.shopping_budget,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableItineraryItem {
    pub id: Uuid,
    pub activity_type: Option<String>,
    pub title: String,
    pub location: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    #[schema(value_type = String, example = "123.45")]
    pub cost: Option<BigDecimal>,
    pub notes: Option<String>,
}

impl From<&(ItineraryItem, Option<Location>, Option<Expense>)> for EncodableItineraryItem {
    fn from(value: &(ItineraryItem, Option<Location>, Option<Expense>)) -> Self {
        let itinerary_item = value.0.clone();
        let location = value.1.clone();
        let expense = value.2.clone();

        Self {
            id: itinerary_item.id,
            activity_type: itinerary_item.activity_type,
            title: itinerary_item.title,
            location: if let Some(l) = location {
                Some(l.address)
            } else {
                None
            },
            start_time: itinerary_item.start_time,
            end_time: itinerary_item.end_time,
            cost: if let Some(e) = expense { e.cost } else { None },
            notes: itinerary_item.notes,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableDocument {
    pub id: Uuid,
    pub filename: String,
    pub size_bytes: i64,
}

impl From<Document> for EncodableDocument {
    fn from(value: Document) -> Self {
        Self {
            id: value.id,
            filename: value.filename,
            size_bytes: value.file_size,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableTripData {
    pub id: Uuid,
    pub title: Option<String>,
    pub banner_image: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub collaborators: Vec<EncodableCollaborator>,
    pub budget_plan: EncodableBudgetPlan,
    pub itinerary: Vec<EncodableItineraryItem>,
    pub documents: Vec<EncodableDocument>,
}

impl From<TripData> for EncodableTripData {
    fn from(value: TripData) -> Self {
        let collaborators: Vec<EncodableCollaborator> = value
            .collaborators
            .iter()
            .map(|c| EncodableCollaborator::from(c))
            .collect();

        let use_personal_budget = value.personal_budget_plan.personal_budget_enabled.clone();

        let budget_plan = EncodableBudgetPlan {
            group_budget: EncodableGroupBudget::from(value.budget_plan),
            personal_budget: EncodablePersonalBudget::from(value.personal_budget_plan),
            expenses: value
                .trip_expenses
                .iter()
                .map(|e| EncodableExpense::from(e))
                .collect(),
            use_personal_budget,
        };

        let itinerary: Vec<EncodableItineraryItem> = value
            .itinerary_items
            .iter()
            .map(|items| EncodableItineraryItem::from(items))
            .collect();

        let documents: Vec<EncodableDocument> = value
            .documents
            .iter()
            .map(|d| EncodableDocument::from(d.clone()))
            .collect();

        Self {
            id: value.trip.id,
            title: value.trip.title,
            banner_image: value.trip.banner_image,
            start_date: value.trip.start_date,
            end_date: value.trip.end_date,
            collaborators,
            budget_plan,
            itinerary,
            documents,
        }
    }
}
