use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use garde::Errors;
use thiserror::Error;

/// Rejection used for [`WithValidation`]
///
/// [`WithValidation`]: crate::WithValidation
#[derive(Debug, Error)]
pub enum WithValidationRejection<T> {
    /// Variant for the extractor's rejection
    #[error(transparent)]
    ExtractionError(T),
    /// Variant for the payload's validation errors. Responds with status code
    /// `422 Unprocessable Content`
    #[error(transparent)]
    ValidationError(#[from] Errors),
}

impl<T: IntoResponse> IntoResponse for WithValidationRejection<T> {
    fn into_response(self) -> Response {
        match self {
            WithValidationRejection::ExtractionError(t) => t.into_response(),
            WithValidationRejection::ValidationError(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, format!("{e}")).into_response()
            }
        }
    }
}
