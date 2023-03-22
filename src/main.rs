#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from you t-rusty server!")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rusty-be", routes![index])
}