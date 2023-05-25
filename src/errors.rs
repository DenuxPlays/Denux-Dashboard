use http::status::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DashboardError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl DashboardError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            DashboardError::NotFound => StatusCode::NOT_FOUND,
            DashboardError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
