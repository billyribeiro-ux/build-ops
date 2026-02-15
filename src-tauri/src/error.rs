use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Dependency not met: {0}")]
    DependencyNotMet(String),

    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Import error: {0}")]
    Import(String),
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}
