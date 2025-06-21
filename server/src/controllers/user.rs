use crate::{
    app::AppState,
    auth::AuthenticatedUser,
    controllers::helper::OkResponse,
    models::user::User,
    util::errors::{AppError, AppResult},
    views::EncodableUser,
};
use actix_web::web::{self, Json};
use diesel::{ExpressionMethods, result::Error::NotFound};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetUsersResponse {
    pub users: Vec<EncodableUser>,
}

#[utoipa::path(
    tag = "users",
    get,
    path = "/api/v1/users",
    responses(
        (status = 200, description = "Successful Response", body = GetUsersResponse),
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_users(
    authenticated: AuthenticatedUser,
    state: web::Data<AppState>,
) -> AppResult<Json<GetUsersResponse>> {
    let mut conn = state.db_connection().await?;

    let result = User::get_all(&mut conn).await;

    match result {
        Ok(users) => {
            let res = users
                .iter()
                .map(|user| EncodableUser {
                    id: user.id,
                    username: user.username.clone(),
                    email: user.email.clone(),
                    avatar: user.avatar.clone(),
                })
                .collect::<Vec<EncodableUser>>();

            Ok(Json(GetUsersResponse { users: res }))
        }
        Err(_) => Err(AppError::InternalError),
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetUserResponse {
    pub user: EncodableUser,
}

#[utoipa::path(
    tag = "users",
    get,
    path = "/api/v1/users/{user_id}",
    responses(
        (status = 200, description = "Successful Response", body = GetUserResponse),
        (status = 404, description = "User Not Found"),
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user(
    authenticated: AuthenticatedUser,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> AppResult<Json<GetUserResponse>> {
    let user_id = path.into_inner();

    let mut conn = state.db_connection().await?;

    let result = User::find(&mut conn, &user_id).await;

    match result {
        Ok(user) => Ok(Json(GetUserResponse {
            user: EncodableUser {
                id: user.id,
                username: user.username,
                email: user.email,
                avatar: user.avatar,
            },
        })),
        Err(NotFound) => Err(AppError::NotFound),
        Err(_) => Err(AppError::InternalError),
    }
}

#[utoipa::path(
    tag = "users",
    delete,
    path = "/api/v1/users/{user_id}",
    responses(
        (status = 200, description = "Successful Response", body = OkResponse),
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_user(
    authenticated: AuthenticatedUser,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> AppResult<OkResponse> {
    let user_id = path.into_inner();

    let mut conn = state.db_connection().await?;

    let result = User::delete(&mut conn, &user_id).await;

    match result {
        Ok(_) => Ok(OkResponse::new()),
        Err(NotFound) => Err(AppError::BadRequest("User not found".to_string())),
        Err(_) => Err(AppError::InternalError),
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateInformationBody {
    #[schema(example = "NewUsername")]
    pub username: Option<String>,
    #[schema(example = "newemail@journly.com")]
    pub email: Option<String>,
}

#[utoipa::path(
    tag = "users",
    put,
    path = "/api/v1/users/{user_id}",
    responses(
        (status = 200, description = "Successful Response", body = OkResponse),
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_user(
    authenticated: AuthenticatedUser,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    new_data: web::Json<UpdateInformationBody>,
) -> AppResult<OkResponse> {
    let user_id = path.into_inner();

    let mut conn = state.db_connection().await?;

    use crate::schema::users::dsl::*;

    if let Some(new_username) = &new_data.username {
        let result = diesel::update(users)
            .filter(id.eq(user_id))
            .set(username.eq(new_username))
            .execute(&mut conn)
            .await;

        if result == Err(NotFound) {
            return Err(AppError::BadRequest("User not found".to_string()));
        } else if result.is_err() {
            return Err(AppError::InternalError);
        }
    }

    if let Some(new_email) = &new_data.email {
        let result = diesel::update(users)
            .filter(id.eq(user_id))
            .set(email.eq(new_email))
            .execute(&mut conn)
            .await;

        if result == Err(NotFound) {
            return Err(AppError::BadRequest("User not found".to_string()));
        } else if result.is_err() {
            return Err(AppError::InternalError);
        }
    }

    Ok(OkResponse::new())
}
