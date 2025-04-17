use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

use crate::{controllers::user_controller, errors::MyError, models::schema::User};

#[get("/users")]
async fn get_users(dp_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = dp_pool.get().await.map_err(MyError::PoolError).unwrap();

    let result: Result<_, _> = user_controller::get_users(&client).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[post("/users")]
async fn add_user(user: web::Json<User>, db_pool: web::Data<Pool>) -> impl Responder {
    let user_info: User = user.into_inner();

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