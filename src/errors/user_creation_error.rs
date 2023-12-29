use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserCreationError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid data")]
    InvalidData,
    #[error("Database error")]
    DatabaseError,
}