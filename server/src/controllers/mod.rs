pub mod user_controller;

pub mod trip_controller;

pub use user_controller::init as init_user_controller;

pub use trip_controller::init as init_trip_controller;