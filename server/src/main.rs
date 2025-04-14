use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use confik::{ Configuration, EnvSource };
use database::{config::ExampleConfig, db::{self, connection_builder}};
use models::user::User;
use errors::MyError;
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;

mod models;
mod errors;
mod database;

#[get("/users")]
async fn get_users(dp_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = dp_pool.get().await.map_err(MyError::PoolError).unwrap();

    let result: Result<_, _> = db::get_users(&client).await;

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
            let result = db::add_user(&client, user_info).await;

            match result {
                Ok(new_user) => HttpResponse::Ok().json(new_user),
                Err(_) => HttpResponse::InternalServerError().into()
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = connection_builder(config.pg).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_users)
            .service(add_user)
    })
        .bind((config.server_addr.clone(), config.dev_port.clone()))?
        .run();

    println!("Server running on {}:{}", config.server_addr, config.dev_port);

    server.await
}
