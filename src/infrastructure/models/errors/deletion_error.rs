#[derive(Debug)]
pub enum DeletionError {
    DatabaseError(sqlx::Error),
    IdNotSpecified,
    NotFoundRecord,
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
            DeletionError::IdNotSpecified => write!(f, "Account ID not specified"),
            DeletionError::NotFoundRecord => write!(f, "No record found for the given ID"),
        }
    }
}
