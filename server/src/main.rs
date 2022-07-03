#[macro_use]
extern crate rocket;
use controllers::user;
use dotenv::dotenv;
mod controllers;
mod models;
mod services;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let _rocket = rocket::build()
        .mount("/user", routes![user::get_user, user::create_user])
        .launch()
        .await?;

    Ok(())
}
