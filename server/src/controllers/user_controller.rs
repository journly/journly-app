use crate::{models::schema::User, AppData};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use utoipa_actix_web::service_config::ServiceConfig;
use uuid::Uuid;

use crate::{
    controllers::log_request,
    models::api::users::{CreateUser, },
};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_user);
    cfg.service(get_users);
    cfg.service(create_user);
    cfg.service(delete_user);
}

const USERS: &str = "users";

#[utoipa::path(
    tag = USERS,
    responses(
        (status = 200, description = "Users were found", body = [User])
    )
)]
#[get("/users")]
async fn get_users(app_data: web::Data<AppData>) -> impl Responder {
    log_request("GET /users", &app_data.connections);

    let result = app_data.db.users.get_users().await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    tag = USERS,
    responses(
        (status = 200, description = "User was created", body = User)
    )
)]
#[post("/users")]
async fn create_user(
    new_user: web::Json<CreateUser>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    log_request("POST /users", &app_data.connections);

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
    responses(
        (status = 200, description = "User was found", body = User)
    )
)]
#[get("/users/{user_id}")]
async fn get_user(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let user_id = path.into_inner();

    log_request(&format!("GET /users/{user_id}"), &app_data.connections);

    let result = app_data.db.users.get_user(user_id).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = USERS,
    responses(
        (status = 200, description = "User was deleted")
    )
)]
#[delete("/users/{user_id}")]
async fn delete_user(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let user_id = path.into_inner();

    log_request(&format!("/users/{user_id}"), &app_data.connections);

    let result = app_data.db.users.delete_user(user_id).await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
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
