use actix_multipart::{form::{json::Json, tempfile::TempFile, MultipartForm}, Multipart};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use argon2::{ password_hash::{ rand_core::OsRng, PasswordHasher, SaltString }, Argon2 };
use deadpool_postgres::Pool;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{errors::MyError, models::{schema::User, users::{AddUser, NewUserDetails}}};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(get_users);
    cfg.service(add_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}

#[get("/users")]
async fn get_users(dp_pool: web::Data<Pool>) -> impl Responder {

    let result = dp_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let stmt = 
            r#"
            SELECT $table_fields FROM public.users;
            "#;

            let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
            let stmt = client.prepare(&stmt).await.unwrap();
        
            let user = client
                .query(&stmt, &[])
                .await
                .unwrap_or_else(|_| Vec::new())
                .iter()
                .map(|row| User::from_row_ref(row).unwrap())
                .collect::<Vec<User>>();
        
            HttpResponse::Ok().json(user)
        }
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[post("/users")]
async fn add_user(new_user: web::Json<AddUser>, db_pool: web::Data<Pool>) -> impl Responder {
    let new_user = new_user.into_inner();

    let result = db_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let stmt = 
            r#"
            INSERT INTO public.users(id, display_name, username, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING $table_fields;
            "#;
            let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
            let stmt = client.prepare(&stmt).await.unwrap();
        
            let salt = SaltString::generate(&mut OsRng);
        
            let argon2 = Argon2::default();
        
            let password_hash = match argon2.hash_password(new_user.password.as_bytes(), &salt) {
                Ok(hash) => hash.to_string(),
                Err(_) => panic!("Failed to hash password.")
            };
            
            let user_id = Uuid::new_v4();
        
            let result = client
                .query(
                    &stmt,
                    &[
                        &user_id,
                        &new_user.display_name,
                        &new_user.username,
                        &password_hash,
                    ],
                )
                .await
                .unwrap_or_else(|_| Vec::new())
                .iter()
                .map(|row| User::from_row_ref(row).unwrap())
                .collect::<Vec<User>>()
                .pop()
                .ok_or(MyError::NotFound); // more applicable for SELECTs

            match result {
                Ok(new_user) => HttpResponse::Ok().json(new_user),
                Err(_) => HttpResponse::InternalServerError().into()
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[get("/users/{user_id}")]
async fn get_user(path: web::Path<Uuid>, db_pool: web::Data<Pool>) -> impl Responder {
    let user_id = path.into_inner();
    
    let result = db_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let stmt = 
            r#"
            SELECT $table_fields FROM public.users WHERE users.id = $user_id;
            "#;
            let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
            let stmt = stmt.replace("$user_id", &format!("'{}'", user_id.to_string()));
            println!("{}", stmt);
            let stmt = client.prepare(&stmt).await.unwrap();

            let result = client
                .query(&stmt, &[])
                .await
                .unwrap_or_else(|_| Vec::new())
                .iter()
                .map(|row| User::from_row_ref(row).unwrap())
                .collect::<Vec<User>>()
                .pop()
                .ok_or(MyError::NotFound); 

            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().body("User not found.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[put("/users/{user_id}")]
async fn update_user(path: web::Path<Uuid>, data: web::Json<NewUserDetails>, db_pool: web::Data<Pool>) -> impl Responder {
    let user_id = path.into_inner();
    let new_user_details = data.into_inner();

    let result = db_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let mut updates: Vec<String> = Vec::new(); 

            if let Some(display_name) = new_user_details.display_name {
                updates.push(format!("display_name = '{}'", display_name).to_string());
            }

            if let Some(password) = new_user_details.password {
                let salt = SaltString::generate(&mut OsRng);

                let argon2 = Argon2::default();

                match argon2.hash_password(password.as_bytes(), &salt) {
                    Ok(hash) => updates.push(format!("password_hash = '{}'", hash.to_string())),
                    Err(_) => panic!("Failed to hash password.")
                }
            }
            
            let stmt = 
            r#"
            UPDATE public.users 
            SET $new_info WHERE id = $user_id
            RETURNING $table_fields;
            "#;
            let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
            let stmt = stmt.replace("$user_id", &format!("'{}'", user_id.to_string()));
            let stmt = stmt.replace("$new_info", &updates.join(", "));
            let stmt = client.prepare(&stmt).await.unwrap();


            let result = client
                .query(&stmt, &[])
                .await
                .unwrap_or_else(|_| Vec::new())
                .iter()
                .map(|row| User::from_row_ref(row).unwrap())
                .collect::<Vec<User>>()
                .pop()
                .ok_or(MyError::NotFound);

            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().body("User not found.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[delete("/users/{user_id}")]
async fn delete_user(path: web::Path<Uuid>, dp_pool: web::Data<Pool>) -> impl Responder {
    let user_id = path.into_inner();

    let result = dp_pool.get().await.map_err(MyError::PoolError);

    match result {
        Ok(client) => {
            let stmt = 
            r#"
            DELETE FROM public.users WHERE id = $user_id
            RETURNING $table_fields;
            "#;
            let stmt = stmt.replace("$user_id", &format!("'{}'", user_id.to_string()));
            let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
            let stmt = client.prepare(&stmt).await.unwrap();

            let result = client.
                query(&stmt, &[])
                .await
                .unwrap_or_else(|_| Vec::new())
                .iter()
                .map(|row| User::from_row_ref(row).unwrap())
                .collect::<Vec<User>>()
                .pop()
                .ok_or(MyError::NotFound);

            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().body("User not found.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().into()
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