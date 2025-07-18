use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
pub struct EncodableCollaborator {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub permission: Option<String>,
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
    pub cost: BigDecimal,
    #[schema(example = "USD")]
    pub currency: String,
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

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct EncodableLocation {
    pub display_name: Option<String>,
    pub address: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ItineraryExpense {
    #[schema(value_type = String, example = "123.45")]
    pub cost: BigDecimal,
    pub currency: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableItineraryItem {
    pub id: Option<Uuid>,
    pub activity_type: Option<String>,
    pub title: Option<String>,
    pub location: Option<EncodableLocation>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub cost: Option<ItineraryExpense>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncodableDocument {
    pub id: Uuid,
    pub filename: String,
    pub size_bytes: i64,
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
