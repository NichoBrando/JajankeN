#[macro_use]
extern crate rocket;
use controllers::user;
use dotenv::dotenv;
use services::auth::users_sessions::initialize_users_sessions;
mod controllers;
mod models;
mod services;
use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

#[options("/<_..>")]
fn all_options() {}
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let _rocket = rocket::build()
    .manage(initialize_users_sessions().await)
    .mount("/", routes![all_options])
    .mount("/user", routes![user::get, user::create, user::login])
    .attach(Cors {})
    .launch()
        .await?;

    Ok(())
}
