pub mod dates;
pub mod trips;
pub mod users;

pub trait ToSql {
    fn to_sql_values(&self) -> String;
}
