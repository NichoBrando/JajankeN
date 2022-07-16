#[macro_use]
extern crate rocket;
use controllers::user;
use dotenv::dotenv;
use services::auth::users_sessions::initialize_users_sessions;
mod controllers;
mod models;
mod services;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let _rocket = rocket::build()
    .manage(initialize_users_sessions().await)
    .mount("/user", routes![user::get, user::create, user::login])
    .launch()
        .await?;

    Ok(())
}
