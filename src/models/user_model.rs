use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: Option<u32>,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub password: String
}