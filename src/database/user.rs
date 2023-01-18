use color_eyre::Result;
use sqlx::{prelude::PgQueryAs, PgPool, Postgres};
use std::sync::Arc;

use crate::{
    config::crypto::CryptoService,
    models::users::{NewUser, User},
};

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUser, crypto_service: &CryptoService) -> Result<User> {
        let password_hash = crypto_service.hash_password(new_user.password).await?;

        let user = sqlx::query_as::<Postgres, User>(
            "insert into users (username, email, password_hash) values ($1, $2, $3) returning *",
        )
        .bind(new_user.username)
        .bind(new_user.email)
        .bind(password_hash)
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }
}
