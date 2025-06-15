#[derive(Debug)]
pub enum RegistrationError {
    DatabaseError(sqlx::Error),
    ValidationError(String),
    InsertionFailed,
}

impl From<sqlx::Error> for RegistrationError {
    fn from(err: sqlx::Error) -> Self {
        RegistrationError::DatabaseError(err)
    }
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationError::DatabaseError(err) => write!(f, "Database error: {}", err),
            RegistrationError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            RegistrationError::InsertionFailed => write!(f, "Insertion failed"),
        }
    }
}
