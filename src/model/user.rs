use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use tracing::log::log;
use uuid::Uuid;
use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::base::DbBmc;
use crate::model::{base, Error, ModelManager, Result};
#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

struct UserForInsert {
    pub username: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    // -- pwd and token info
    pub pwd: Option<String>, // encrypted, #_scheme_id#...
    pub pwd_salt: Uuid,
    pub token_salt: Uuid
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    // -- token info
    pub token_salt: Uuid
}

/// Marker trait
pub trait UserBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "\"user\"";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy
    {
        base::get::<Self, E>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        tracing::info!("{}", Self::TABLE.to_string());

        let db = mm.db();
        let sql = format!("SELECT * FROM {} WHERE username = $1", Self::TABLE);
        tracing::info!("{}", sql);

        let user = sqlx::query_as(&sql)
            .bind(username)
            .fetch_optional(db)
            .await?; // get the task from the Optional;


        Ok(user)
    }

    pub async fn update_pw(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        pwd_clear: &str
    ) -> Result<()> {
        let db = mm.db();

        let user: UserForLogin = Self::get(ctx, mm, id).await?;
        let pwd = pwd::encrypt_pw(&EncryptContent {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt.to_string(),
        })?;

        // make the query
        let sql_query = format!("UPDATE {} SET pwd = $1 WHERE id = $2", Self::TABLE);
        sqlx::query(&sql_query)
            .bind(&pwd)
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

// region:      --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::{Context, Result};
    use serial_test::serial;


    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::INFO)
            .try_init();

        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        // -- Exec
        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        // -- Check
        assert_eq!(user.username, fx_username);

        Ok(())
    }
}

// endregion:   --- Tests
