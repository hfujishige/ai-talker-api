#[derive(Debug)]
pub enum DeletionError {
    DatabaseError(sqlx::Error),
    ValidationError(String),
    DeletionFailed,
}

impl From<sqlx::Error> for DeletionError {
    fn from(err: sqlx::Error) -> Self {
        DeletionError::DatabaseError(err)
    }
}

impl std::fmt::Display for DeletionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeletionError::DatabaseError(err) => write!(f, "Database error: {}", err),
            DeletionError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DeletionError::DeletionFailed => write!(f, "Insertion failed"),
        }
    }
}
