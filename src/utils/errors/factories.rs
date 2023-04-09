use super::base::{ErrorType, MappedErrors};

/// A factory for creation errors
pub fn creation_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::CreationError))
}

/// A factory for updating errors
pub fn updating_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::UpdatingError))
}

/// A factory for fetching errors
pub fn fetching_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::FetchingError))
}

/// A factory for deletion errors
pub fn deletion_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::DeletionError))
}

/// A factory for use case errors
pub fn use_case_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::UseCaseError))
}

/// A factory for execution errors
pub fn execution_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::ExecutionError))
}

/// A factory for invalid repository errors
pub fn invalid_repo_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(
        msg,
        exp,
        prev,
        ErrorType::InvalidRepositoryError,
    ))
}

/// A factory for invalid argument errors
pub fn invalid_arg_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(
        msg,
        exp,
        prev,
        ErrorType::InvalidArgumentError,
    ))
}
