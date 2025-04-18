use actix_multipart::{form::{json::Json, tempfile::TempFile, MultipartForm}, Multipart};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;
use uuid::Uuid;

use crate::{controllers::user_controller, errors::MyError, models::users::{AddUser, NewUserDetails}};

#[get("")]
async fn get_users(dp_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = dp_pool.get().await.map_err(MyError::PoolError).unwrap();

    let result: Result<_, _> = user_controller::get_users(&client).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[post("")]
async fn add_user(user: web::Json<AddUser>, db_pool: web::Data<Pool>) -> impl Responder {
    let user_info = user.into_inner();

    let result = db_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let result = user_controller::add_user(&client, user_info).await;

            match result {
                Ok(new_user) => HttpResponse::Ok().json(new_user),
                Err(_) => HttpResponse::InternalServerError().into()
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[get("/{user_id}")]
async fn get_user(path: web::Path<Uuid>, db_pool: web::Data<Pool>) -> impl Responder {
    let user_id = path.into_inner();
    
    let result = db_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let result = user_controller::get_user(&client, user_id).await;

            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().body("User not found.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[put("/{user_id}")]
async fn update_user(path: web::Path<Uuid>, data: web::Json<NewUserDetails>, db_pool: web::Data<Pool>) -> impl Responder {
    let user_id = path.into_inner();
    let new_user_details = data.into_inner();

    let result = db_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let result = user_controller::update_user_details(&client, new_user_details, user_id).await;

            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().body("User not found.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[delete("/{user_id}")]
async fn delete_user(path: web::Path<Uuid>, dp_pool: web::Data<Pool>) -> impl Responder {
    let user_id = path.into_inner();

    let result = dp_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let result = user_controller::delete_user(&client, user_id).await;

            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().body("User not found.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[get("/{user_id}/profile_picture")]
async fn get_profile_picture(path: web::Path<Uuid>, dp_pool: web::Data<Pool>) -> impl Responder {
    let user_id = path.into_inner();

    let result 
}

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "10MB")]
    file: TempFile,
    json: Json<Metadata>
}

#[post("/{user_id}/profile_picture")]
async fn set_profile_picture(
    path: web::Path<Uuid>, 
    MultipartForm(form): MultipartForm<UploadForm>, 
    dp_pool: web::Data<Pool>) -> impl Responder 
{
    let user_id = path.into_inner();

    Ok() 
}