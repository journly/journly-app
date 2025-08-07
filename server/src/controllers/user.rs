use crate::{
    app::AppState,
    auth::AuthenticatedUser,
    controllers::helper::OkResponse,
    models::user::User,
    s3_client::get_file_extension,
    util::errors::{AppError, AppResult, ErrorResponse},
    views::EncodableUser,
};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::web::{self, Json};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose};
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
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ErrorResponse)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_users(
    authenticated: AuthenticatedUser,
    state: web::Data<AppState>,
) -> AppResult<Json<GetUsersResponse>> {
    if !authenticated.is_admin() {
        return Err(AppError::Forbidden("Insufficient permissions."));
    }

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
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ErrorResponse),
        (status = 404, description = "User Not Found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
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
    if !authenticated.is_admin() {
        return Err(AppError::Forbidden("Insufficient permissions"));
    }

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
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
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

    if !authenticated.is_admin() && authenticated.user.id != user_id {
        return Err(AppError::Forbidden("Insufficient permissions"));
    }

    let mut conn = state.db_connection().await?;

    let result = User::delete(&mut conn, &user_id).await;

    match result {
        Ok(_) => Ok(OkResponse::new()),
        Err(NotFound) => Err(AppError::NotFound),
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
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Conflicts with existing resource", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
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

    if !authenticated.is_admin() && authenticated.user.id != user_id {
        return Err(AppError::Forbidden("Insufficient permissions"));
    }

    let mut conn = state.db_connection().await?;

    use crate::schema::users::dsl::*;

    if let Some(new_username) = &new_data.username {
        let result = diesel::update(users)
            .filter(id.eq(user_id))
            .set(username.eq(new_username))
            .execute(&mut conn)
            .await;

        if result == Err(NotFound) {
            return Err(AppError::NotFound);
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
            return Err(AppError::NotFound);
        } else if result.is_err() {
            return Err(AppError::Conflict);
        }
    }

    Ok(OkResponse::new())
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PasswordUpdateRequest {
    pub current_password: String,
    pub new_password: String,
}

#[utoipa::path(
    tag = "users",
    put,
    path = "/api/v1/users/{user_id}/password",
    responses(
        (status = 200, description = "Password updated successfully", body = OkResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    security(("jwt" = []))
)]
pub async fn update_user_password(
    authenticated: AuthenticatedUser,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<PasswordUpdateRequest>,
) -> AppResult<OkResponse> {
    let user_id = path.into_inner();
    let user = authenticated.user;

    if user.id != user_id {
        return Err(AppError::Forbidden("Insufficient permissions"));
    }

    let PasswordUpdateRequest {
        current_password,
        new_password,
    } = body.into_inner();

    let mut conn = state.db_connection().await?;

    let user_password_salt = user.password_salt;
    let user_password_hash = user.password_hash;

    if user_password_hash.is_none() || user_password_salt.is_none() {
        return Err(AppError::InternalError);
    }

    let salt = match SaltString::from_b64(
        &general_purpose::STANDARD_NO_PAD.encode(user_password_salt.unwrap()),
    ) {
        Ok(res) => res,
        _ => return Err(AppError::InternalError),
    };

    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(current_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        _ => return Err(AppError::InternalError),
    };

    if password_hash == user_password_hash.unwrap() {
        use crate::schema::users;

        let salt = SaltString::generate(&mut OsRng);
        let new_salt_bytes: Vec<u8> = general_purpose::STANDARD_NO_PAD
            .decode(salt.as_str())
            .unwrap();

        let argon2 = Argon2::default();

        let new_password_hash: String;
        if let Ok(hash) = argon2.hash_password(new_password.as_bytes(), &salt) {
            new_password_hash = hash.to_string();
        } else {
            return Err(AppError::InternalError);
        }

        let update_result = diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .set((
                users::password_hash.eq(new_password_hash),
                users::password_salt.eq(new_salt_bytes),
            ))
            .execute(&mut conn)
            .await;

        return match update_result {
            Ok(_) => Ok(OkResponse::new()),
            Err(_) => Err(AppError::InternalError),
        };
    }

    Err(AppError::NotFound)
}

#[derive(Debug, MultipartForm, ToSchema)]
pub struct UploadForm {
    #[multipart(limit = "1MB")]
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub file: TempFile,
}

#[utoipa::path(
    tag = "users",
    put,
    path = "/api/v1/users/{user_id}/profile-picture",
    summary = "Upload or replace a user's profile picture",
    params(
        ("user_id" = Uuid, Path, description = "ID of the user whose profile picture is being changed")
    ),
    request_body(
        content = UploadForm,
        content_type = "multipart/form-data",
        description = "Image file to upload as profile picture",
    ),
    responses(
        (status = 200, description = "Profile picture updated successfully", body = OkResponse),
        (status = 400, description = "Invalid file", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Update conflict", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    security(("jwt" = []))
)]
pub async fn change_profile_picture(
    authenticated: AuthenticatedUser,
    path: web::Path<Uuid>,
    MultipartForm(form): MultipartForm<UploadForm>,
    state: web::Data<AppState>,
) -> AppResult<OkResponse> {
    let user_id = path.into_inner();
    let user = authenticated.user;

    if user.id != user_id {
        return Err(AppError::Forbidden("Insufficient permissions"));
    }

    let s3_client = match &state.s3 {
        Some(s3) => s3,
        None => return Err(AppError::InternalError),
    };

    let mut conn = state.db_connection().await?;

    let file = form.file;

    let content_type = file.content_type.clone();

    if content_type.is_none() {
        return Err(AppError::BadRequest("Invalid file type."));
    };

    match content_type.unwrap().type_() {
        mime::IMAGE => {
            let key_prefix = "pfp";

            // upload new pfp
            let mut file_ext = get_file_extension(&file);

            println!("extension {file_ext}");

            if file_ext == "jpg" {
                file_ext = "jpeg".to_string();
            }

            println!("extension now {file_ext}");

            if file_ext != "png" && file_ext != "jpeg" && file_ext != "webp" {
                return Err(AppError::BadRequest("Invalid file type."));
            }

            let profile_picture_url = s3_client
                .upload(&file, key_prefix, &format!("image/{}", file_ext))
                .await;

            use crate::schema::users::dsl::*;

            let result = diesel::update(users)
                .filter(id.eq(user_id))
                .set(avatar.eq(profile_picture_url))
                .execute(&mut conn)
                .await;

            if result == Err(NotFound) {
                return Err(AppError::NotFound);
            } else if result.is_err() {
                return Err(AppError::Conflict);
            }

            // delete old pfp
            if user.avatar.is_some() {
                let key = s3_client.get_key_from_url(&user.avatar.unwrap());

                s3_client.delete_file(&key).await;
            }

            Ok(OkResponse::new())
        }
        shit => {
            println!("here {shit}");

            Err(AppError::BadRequest("Invalid file type."))
        }
    }
}
