use std::sync::{Arc, Mutex};

use crate::database::db::Database;

pub struct AppData {
    pub db: Arc<Database>,
    pub connections: Mutex<u32>
}
