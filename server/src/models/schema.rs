use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use tokio_pg_mapper_derive::PostgresMapper;
use typeshare::typeshare;
use utoipa::ToSchema;
use uuid::Uuid;

/// --- Taken from rust-postgres repository ---
///  
/// The following implementations are provided by this crate, along with the
/// corresponding Postgres types:
///
/// | Rust type                         | Postgres type(s)                              |
/// |-----------------------------------|-----------------------------------------------|
/// | `bool`                            | BOOL                                          |
/// | `i8`                              | "char"                                        |
/// | `i16`                             | SMALLINT, SMALLSERIAL                         |
/// | `i32`                             | INT, SERIAL                                   |
/// | `u32`                             | OID                                           |
/// | `i64`                             | BIGINT, BIGSERIAL                             |
/// | `f32`                             | REAL                                          |
/// | `f64`                             | DOUBLE PRECISION                              |
/// | `&str`/`String`                   | VARCHAR, CHAR(n), TEXT, CITEXT, NAME, UNKNOWN |
/// |                                   | LTREE, LQUERY, LTXTQUERY                      |
/// | `&[u8]`/`Vec<u8>`                 | BYTEA                                         |
/// | `HashMap<String, Option<String>>` | HSTORE                                        |
/// | `SystemTime`                      | TIMESTAMP, TIMESTAMP WITH TIME ZONE           |
/// | `IpAddr`                          | INET                                          |
///
/// In addition, some implementations are provided for types in third party
/// crates. These are disabled by default; to opt into one of these
/// implementations, activate the Cargo feature corresponding to the crate's
/// name prefixed by `with-`. For example, the `with-serde_json-1` feature enables
/// the implementation for the `serde_json::Value` type.
///
/// | Rust type                       | Postgres type(s)                    |
/// |---------------------------------|-------------------------------------|
/// | `chrono::NaiveDateTime`         | TIMESTAMP                           |
/// | `chrono::DateTime<Utc>`         | TIMESTAMP WITH TIME ZONE            |
/// | `chrono::DateTime<Local>`       | TIMESTAMP WITH TIME ZONE            |
/// | `chrono::DateTime<FixedOffset>` | TIMESTAMP WITH TIME ZONE            |
/// | `chrono::NaiveDate`             | DATE                                |
/// | `chrono::NaiveTime`             | TIME                                |
/// | `time::PrimitiveDateTime`       | TIMESTAMP                           |
/// | `time::OffsetDateTime`          | TIMESTAMP WITH TIME ZONE            |
/// | `time::Date`                    | DATE                                |
/// | `time::Time`                    | TIME                                |
/// | `eui48::MacAddress`             | MACADDR                             |
/// | `geo_types::Point<f64>`         | POINT                               |
/// | `geo_types::Rect<f64>`          | BOX                                 |
/// | `geo_types::LineString<f64>`    | PATH                                |
/// | `serde_json::Value`             | JSON, JSONB                         |
/// | `uuid::Uuid`                    | UUID                                |
/// | `bit_vec::BitVec`               | BIT, VARBIT                         |
/// | `eui48::MacAddress`             | MACADDR                             |
/// | `cidr::InetCidr`                | CIDR                                |
/// | `cidr::InetAddr`                | INET                                |
/// | `smol_str::SmolStr`             | VARCHAR, CHAR(n), TEXT, CITEXT,     |
/// |                                 | NAME, UNKNOWN, LTREE, LQUERY,       |
/// |                                 | LTXTQUERY                           |

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize, ToSchema)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub profile_picture_url: Option<String>,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "user_trips")]
pub struct UserTrip {
    pub trip_id: Uuid,
    pub user_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "trips")]
pub struct Trip {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub image_url: Option<String>,
    pub dates_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "dates")]
pub struct Dates {
    pub id: Uuid,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "sections")]
pub struct Section {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub title: String,
    pub order_rank: i16,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "itineraries")]
pub struct Itinerary {
    pub id: Uuid,
    pub widget_id: Uuid,
    pub dates_id: Uuid,
    pub map_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "widgets")]
pub struct Widget {
    pub id: Uuid,
    pub section_id: Uuid,
    pub widget_type: String,
    pub order_rank: i16,
    pub width: i16,
    pub height: i16,
    pub content: serde_json::Value,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "itinerary_activities")]
pub struct ItineraryActivity {
    pub id: Uuid,
    pub itinerary_id: Uuid,
    pub activity_type: String,
    pub date_id: Uuid,
    pub coordinates_id: Option<Uuid>,
    pub start_time: Option<PrimitiveDateTime>,
    pub end_time: Option<PrimitiveDateTime>,
    pub expense_id: Option<Uuid>,
    pub notes: String,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "attachments")]
pub struct Attachment {
    pub trip_id: Uuid,
    pub file_id: Uuid,
    pub activity_id: Option<Uuid>,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "budgeting_trackers")]
pub struct BudgetingTracker {
    pub id: Uuid,
    pub widget_id: Uuid,
    pub total_budget: Decimal,
    pub currency: String,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "expense")]
pub struct Expense {
    pub expense_id: Uuid,
    pub budgeting_tracker_id: Uuid,
    pub title: String,
    pub cost: Decimal,
    pub expense_type: String,
    pub split_type: String,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "files")]
pub struct File {
    pub file_id: Uuid,
    pub user_id: Uuid,
    pub file_name: String,
    pub file_url: String,
    pub file_hash: String,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "maps")]
pub struct Map {
    pub map_id: Uuid,
    pub user_id: Uuid,
    pub map_type: String,
    pub title: String,
    pub coordinates_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "coordinates")]
pub struct Coordinate {
    pub coordinate_id: Uuid,
    pub longitude: f64,
    pub latitude: f64,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "markers")]
pub struct Marker {
    pub marker_id: Uuid,
    pub coordinates_id: Uuid,
    pub activity_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "expense_payers")]
pub struct ExpensePayer {
    pub expense_payer_id: Uuid,
    pub user_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "journals")]
pub struct Journal {
    pub journal_id: Uuid,
    pub owner_id: Uuid,
    pub content: String,
    pub last_edit: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "user_journals")]
pub struct UserJournal {
    pub user_id: Uuid,
    pub journal_id: Uuid,
}
