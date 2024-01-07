use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UserProfile {
    pub login: String,
}
