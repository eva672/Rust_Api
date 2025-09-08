use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Not found")]
    NotFound,
    
    #[error("Internal server error")]
    InternalServerError,
}

impl warp::reject::Reject for AppError {}