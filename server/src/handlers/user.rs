use crate::{
    models::api::users::{UserDisplayName, UserEmail},
    AppData,
};
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use uuid::Uuid;

use crate::models::api::users::{CreateUser, User};

const USERS: &str = "users";

#[utoipa::path(
    tag = USERS,
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "Users were found", body = [User])
    )
)]
pub async fn get_users(app_data: web::Data<AppData>) -> impl Responder {
    let result = app_data.db.users.get_users().await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    tag = USERS,
    post,
    path = "/api/users",
    responses(
        (status = 200, description = "User was created", body = User)
    )
)]
pub async fn create_user(
    new_user: web::Json<CreateUser>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let new_user = new_user.into_inner();

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash: String;
    if let Ok(hash) = argon2.hash_password(new_user.password.as_bytes(), &salt) {
        password_hash = hash.to_string();
    } else {
        return HttpResponse::InternalServerError().finish();
    }

    let result = app_data
        .db
        .users
        .add_user(new_user.username, password_hash)
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = USERS,
    get,
    path = "/api/users/{user_id}",
    responses(
        (status = 200, description = "User was found", body = User)
    )
)]
pub async fn get_user(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let user_id = path.into_inner();

    let result = app_data.db.users.get_user(user_id).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = USERS,
    delete,
    path = "/api/users/{user_id}",
    responses(
        (status = 200, description = "User was deleted")
    )
)]
pub async fn delete_user(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let user_id = path.into_inner();

    let result = app_data.db.users.delete_user(user_id).await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[utoipa::path(
    tag = USERS,
    get,
    path = "/api/users/{user_id}/display_name",
    responses(
        (status = 200, description = "User display name was found", body = str)
    )
)]
pub async fn get_user_display_name(
    path: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = app_data.db.users.get_user(user_id).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user.display_name),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = USERS,
    put,
    path = "/api/users/{user_id}/display_name",
    responses(
        (status = 200, description = "User display name was updated", body = str)
    )
)]
pub async fn update_user_display_name(
    path: web::Path<Uuid>,
    new_display_name: web::Json<UserDisplayName>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let user_id = path.into_inner();

    let new_display_name = new_display_name.display_name.clone();

    let result = app_data
        .db
        .users
        .update_user_display_name(user_id, new_display_name)
        .await;

    match result {
        Ok(display_name) => HttpResponse::Ok().json(display_name),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = USERS,
    get,
    path = "/api/users/{user_id}/email",
    responses(
        (status = 200, description = "User email was found", body = str)
    )
)]
pub async fn get_user_email(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let user_id = path.into_inner();

    let result = app_data.db.users.get_user(user_id).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user.email),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = USERS,
    put,
    path = "/api/users/{user_id}/email",
    responses(
        (status = 200, description = "User email was updated", body = str)
    )
)]
pub async fn update_user_email(
    path: web::Path<Uuid>,
    new_email: web::Json<UserEmail>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let user_id = path.into_inner();

    let new_email = new_email.email.clone();

    let result = app_data
        .db
        .users
        .update_user_email(user_id, new_email)
        .await;

    match result {
        Ok(email) => HttpResponse::Ok().json(email),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// #[get("/{user_id}/profile_picture")]
// async fn get_profile_picture(path: web::Path<Uuid>, dp_pool: web::Data<Pool>) -> impl Responder {
//     let user_id = path.into_inner();

//     let result
// }

// #[derive(Debug, Deserialize)]
// struct Metadata {
//     name: String
// }

// #[derive(Debug, MultipartForm)]
// struct UploadForm {
//     #[multipart(limit = "10MB")]
//     file: TempFile,
//     json: Json<Metadata>
// }

// #[post("/{user_id}/profile_picture")]
// async fn set_profile_picture(
//     path: web::Path<Uuid>,
//     MultipartForm(form): MultipartForm<UploadForm>,
//     dp_pool: web::Data<Pool>) -> impl Responder
// {
//     let user_id = path.into_inner();

//     Ok()
// }
