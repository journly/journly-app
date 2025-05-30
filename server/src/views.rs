use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::trip::Trip;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableUser {
    pub id: Uuid,
    #[schema(example = "journlyuser223")]
    pub username: String,
    #[schema(example = "funemail@journly.com")]
    pub email: String,
    pub avatar: Option<String>,
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableItineraryActivity {
    pub id: Uuid,
    pub activity_type: Option<String>,
    pub title: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    #[schema(value_type = String, example = "123.45")]
    pub cost: Option<BigDecimal>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableItinerary {
    pub activities: Vec<EncodableItineraryActivity>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableDocument {
    pub id: Uuid,
    pub filename: String,
    pub size_bytes: i64,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableDocuments {
    pub documents: Vec<EncodableDocuments>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableTripDetails {
    pub id: Uuid,
    pub title: Option<String>,
    pub banner_image: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub collaborators: Vec<EncodableUser>,
    pub budget_plan: EncodableBudgetPlan,
    pub itinerary: EncodableItinerary,
    pub documents: EncodableDocuments,
}
