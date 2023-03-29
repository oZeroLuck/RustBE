mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket::http::Status;
use rocket::serde::json::Json;

use utoipa_swagger_ui::SwaggerUi;
use utoipa::{OpenApi};

use api::user_api::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        index,
        api::user_api::get_user,
        api::user_api::create_user,
    ),
    components(
        schemas(models::user_model::User)
    ),
    tags(
        (name = "User", description = "User management endpoints."),
        (name = "Base", description = "Test base endpoints.")
    )
)]
struct ApiDoc;

#[utoipa::path(
    tag = "Base",
    context_path = "/rusty-be",
    responses(
        (status = 200, description = "Get hello world")
    )
)]
#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from you t-rusty server!")))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/",
               SwaggerUi::new("/swagger-ui/<_..>")
                   .url("/api-doc/openapi.json", ApiDoc::openapi()))
        .mount(
            "/rusty-be",
            routes![
                index,
            ])
        .mount(
            "/rusty-be/user",
            routes![
                get_user,
                create_user
            ]
        )
}