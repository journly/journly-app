use diesel_async::AsyncPgConnection;

use crate::models::user::{LoggedUser, User};

use super::errors::AppError;

pub async fn validate_admin_user(
    logged_user: &LoggedUser,
    conn: &mut AsyncPgConnection,
) -> Result<(), AppError> {
    let user_id = logged_user.id;

    match User::find(conn, &user_id).await {
        Ok(user) => {
            if user.is_admin {
                Ok(())
            } else {
                Err(AppError::Unauthorized)
            }
        }
        Err(_) => Err(AppError::Unauthorized),
    }
}
