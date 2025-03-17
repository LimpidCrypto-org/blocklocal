use thiserror::Error;

pub type DbResult<T> = std::result::Result<T, DbError>;

#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    DbError(#[from] sea_orm::error::DbErr),
}
