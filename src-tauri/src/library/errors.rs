use thiserror::Error;

use crate::library_db::errors::LibraryDbError;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("invalid roots")]
    InvalidRoots,

    #[error(transparent)]
    Db(#[from] LibraryDbError),
}
