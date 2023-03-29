mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;

use api::user_api::*;

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from you t-rusty server!")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/rusty-be",
            routes![index, get_user, create_user])
}