use crate::ValidationError;

/// The validation result.
pub type ValidationResult<T, S> = Result<T, ValidationError<S>>;
