use sqlx::FromRow;
use sqlx::postgres::PgRow;
use crate::ctx::Ctx;
use crate::model::{Error, ModelManager, Result};
use crate::model::task::Task;

pub trait DbBmc {
    const TABLE: &'static str;
}

/// MC: model controller
/// E: the entity that the function returns
pub async fn get<MC, E>(
    _ctx: &Ctx,
    mm: &ModelManager,
    id: i64
) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();
    // Define the query
    let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);

    let entity: E = sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(db)
        .await? // select statement will fail with `?` if problem
        .ok_or(Error::EntityNotFound { entity: MC::TABLE, id})?; // get the task from the Optional;

    Ok(entity)
}