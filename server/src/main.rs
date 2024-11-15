use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
mod models;
mod schema;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}
async fn create_user(user_data: web::Json<models::NewUser<'_>>) -> impl Responder {
    use crate::schema::users;
    let mut conn = establish_connection(); // Bağlantıyı mutable yapıyoruz
    let new_user = models::NewUser {
        username: &user_data.username,
        password_hash: &user_data.password_hash,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn) // Mutable bağlantıyı buraya gönderiyoruz
        .expect("Error saving new user");
    HttpResponse::Created().json(new_user)
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    actix_web::rt::System::new().block_on(async {
        HttpServer::new(|| {
            App::new().wrap(Cors::permissive()).route(
                "/",
                web::get().to(|| async { "Welcome to Finance Health API!" }),
            )
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    })
}

// fn main() {
//     println!("omer")
// }
