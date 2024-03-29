use super::base::{ErrorType, MappedErrors};

/// A factory for creation errors
pub fn creation_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg).with_error_type(ErrorType::CreationError)
}

/// A factory for updating errors
pub fn updating_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg).with_error_type(ErrorType::UpdatingError)
}

/// A factory for fetching errors
pub fn fetching_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg).with_error_type(ErrorType::FetchingError)
}

/// A factory for deletion errors
pub fn deletion_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg).with_error_type(ErrorType::DeletionError)
}

/// A factory for use case errors
pub fn use_case_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg).with_error_type(ErrorType::UseCaseError)
}

/// A factory for execution errors
pub fn execution_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg).with_error_type(ErrorType::ExecutionError)
}

/// A factory for invalid repository errors
pub fn invalid_repo_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg)
        .with_error_type(ErrorType::InvalidRepositoryError)
}

/// A factory for invalid argument errors
pub fn invalid_arg_err(msg: String) -> MappedErrors {
    MappedErrors::default(msg).with_error_type(ErrorType::InvalidArgumentError)
}

// * ---------------------------------------------------------------------------
// * TESTS
// * ---------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::errors::{base::ErrorType, ErrorCodes};

    #[test]
    fn test_default_factories() {
        assert_eq!(
            creation_err("create".to_string()).error_type(),
            ErrorType::CreationError
        );

        assert_eq!(
            updating_err("update".to_string()).error_type(),
            ErrorType::UpdatingError
        );

        assert_eq!(
            fetching_err("fetch".to_string()).error_type(),
            ErrorType::FetchingError
        );

        assert_eq!(
            deletion_err("delete".to_string()).error_type(),
            ErrorType::DeletionError
        );

        assert_eq!(
            use_case_err("use_case".to_string()).error_type(),
            ErrorType::UseCaseError
        );

        assert_eq!(
            execution_err("execution".to_string()).error_type(),
            ErrorType::ExecutionError
        );

        assert_eq!(
            invalid_repo_err("invalid_repo".to_string()).error_type(),
            ErrorType::InvalidRepositoryError
        );

        assert_eq!(
            invalid_arg_err("invalid_arg".to_string()).error_type(),
            ErrorType::InvalidArgumentError
        );
    }

    #[test]
    fn test_creation_error_factory() {
        fn result_function() -> Result<String, MappedErrors> {
            creation_err("create".to_string())
                .with_code("ID001")
                .with_code("ID002")
                .as_error()
        }

        let result = result_function().unwrap_err();

        assert!(
            result.code() ==
                ErrorCodes::Codes(vec![
                    "ID001".to_string(),
                    "ID002".to_string()
                ])
        );
    }
}
