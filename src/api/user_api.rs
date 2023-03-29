use rocket::serde::json::Json;
use crate::models::user_model::User;
use rocket::http::Status;

#[utoipa::path(
    tag = "User",
    context_path = "/rusty-be/user",
    params(
        ("id" = u32, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "Get user by uid", body = User),
        (status = 400, description = "Uid not valid")
    )
)]
#[get("/<id>")]
pub fn get_user(id: u32) -> Result<Json<User>, Status> {
    let user = User {
        id: Some(1),
        name: "Marco".to_string(),
        surname: "Rossi".to_string(),
        username: "marco.rossi@gmail.com".to_string(),
        password: "Password".to_string()
    };

    match id {
        1 => Ok(Json(user)),
        _ => Err(Status::BadRequest)
    }
}

#[utoipa::path(
    tag = "User",
    context_path = "/rusty-be/user",
    request_body = User,
    responses(
        (status = 200, description = "Created user", body = User)
    )
)]
#[post("/", data = "<new_user>")]
pub fn create_user(new_user: Json<User>) -> Result<Json<User>, Status> {
    let created = User {
        id: None,
        name: new_user.name.to_owned(),
        surname: new_user.surname.to_owned(),
        username: new_user.username.to_owned(),
        password: new_user.password.to_owned()
    };

    Ok(Json(created))
}