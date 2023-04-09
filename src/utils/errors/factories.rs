use super::base::{ErrorType, MappedErrors};

pub fn creation_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::CreationError))
}

pub fn updating_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::UpdatingError))
}

pub fn fetching_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::FetchingError))
}

pub fn deletion_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::DeletionError))
}

pub fn use_case_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::UseCaseError))
}

pub fn execution_err<T>(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> Result<T, MappedErrors> {
    Err(MappedErrors::new(msg, exp, prev, ErrorType::ExecutionError))
}

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
