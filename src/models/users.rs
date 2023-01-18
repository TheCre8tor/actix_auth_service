use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Validate, Debug)]
pub struct NewUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct UpdateProfile {
    pub full_name: Option<String>,
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
}
