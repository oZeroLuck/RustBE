use rocket::serde::json::Json;
use crate::models::user_model::User;
use rocket::http::Status;

#[get("/user/<id>")]
pub fn get_user(id: i32) -> Result<Json<User>, Status> {
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

#[post("/user", data = "<new_user>")]
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