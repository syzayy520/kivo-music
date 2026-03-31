use thiserror::Error;

/// Library DB result type.
pub type LibraryDbResult<T> = Result<T, LibraryDbError>;

#[derive(Debug, Error)]
pub enum LibraryDbError {
    #[error("path resolve error while {context}: {source}")]
    PathResolve {
        context: &'static str,
        #[source]
        source: tauri::Error,
    },

    #[error("io error while {context}: {source}")]
    Io {
        context: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error("sqlite error while {context}: {source}")]
    Sql {
        context: &'static str,
        #[source]
        source: rusqlite::Error,
    },

    #[error("migration error: {0}")]
    Migration(String),
}
