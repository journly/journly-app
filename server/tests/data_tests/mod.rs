use deadpool_postgres::Pool;
use journaly_server::database::db::Database;

use crate::{init_db_context, init_pg_pool};

#[cfg(test)]
pub mod db_context_test;

#[cfg(test)]
pub mod trip_dao_test;

