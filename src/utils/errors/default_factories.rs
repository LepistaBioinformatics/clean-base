use super::base::{ErrorType, MappedErrors};

// ? ---------------------------------------------------------------------------
// ? FACTORY FUNCTIONS
// Such functions should ve used over the raw `MappedErrors` struct to resolve
// specific errors.
// ? ---------------------------------------------------------------------------

pub fn creation_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::CreationError)
}

pub fn updating_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::UpdatingError)
}

pub fn fetching_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::FetchingError)
}

pub fn deletion_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::DeletionError)
}

pub fn use_case_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::UseCaseError)
}

pub fn execution_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::ExecutionError)
}

pub fn invalid_repo_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::InvalidRepositoryError)
}

pub fn invalid_arg_err(
    msg: String,
    exp: Option<bool>,
    prev: Option<MappedErrors>,
) -> MappedErrors {
    MappedErrors::new(msg, exp, prev, ErrorType::InvalidArgumentError)
}

// ? ---------------------------------------------------------------------------
// ? TESTS
// ? ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        creation_err, deletion_err, fetching_err, updating_err, ErrorType,
    };

    #[test]
    fn creation_err_works() {
        assert_eq!(
            creation_err("msg".to_string(), None, None).error_type(),
            ErrorType::CreationError
        );
    }

    #[test]
    fn deletion_err_works() {
        assert_eq!(
            deletion_err("msg".to_string(), None, None).error_type(),
            ErrorType::DeletionError
        );
    }

    #[test]
    fn fetching_err_works() {
        assert_eq!(
            fetching_err("msg".to_string(), None, None).error_type(),
            ErrorType::FetchingError
        );
    }

    #[test]
    fn updating_err_works() {
        assert_eq!(
            updating_err("msg".to_string(), None, None).error_type(),
            ErrorType::UpdatingError
        );
    }
}
