mod base;

/// This module contains the MappedErrors `Results` factories used to construct
/// errors. These factories are used to standardize errors codes.
pub mod factories;

/// This module contains the MappedErrors simple factories used to construct
/// errors. These factories are used to standardize errors codes.
mod default_factories;
pub use default_factories::*;

// * ---------------------------------------------------------------------------
// * TESTS
// * ---------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::factories;
    use crate::utils::errors::base::ErrorType;

    #[test]
    fn test_creation_err() {
        assert_eq!(
            factories::creation_err::<()>("create".to_string(), None, None)
                .unwrap_err()
                .error_type(),
            ErrorType::CreationError
        );

        assert_eq!(
            factories::updating_err::<()>("update".to_string(), None, None)
                .unwrap_err()
                .error_type(),
            ErrorType::UpdatingError
        );

        assert_eq!(
            factories::fetching_err::<()>("fetch".to_string(), None, None)
                .unwrap_err()
                .error_type(),
            ErrorType::FetchingError
        );

        assert_eq!(
            factories::deletion_err::<()>("delete".to_string(), None, None)
                .unwrap_err()
                .error_type(),
            ErrorType::DeletionError
        );

        assert_eq!(
            factories::use_case_err::<()>("use_case".to_string(), None, None)
                .unwrap_err()
                .error_type(),
            ErrorType::UseCaseError
        );

        assert_eq!(
            factories::execution_err::<()>("execution".to_string(), None, None)
                .unwrap_err()
                .error_type(),
            ErrorType::ExecutionError
        );

        assert_eq!(
            factories::invalid_repo_err::<()>(
                "invalid_repo".to_string(),
                None,
                None
            )
            .unwrap_err()
            .error_type(),
            ErrorType::InvalidRepositoryError
        );

        assert_eq!(
            factories::invalid_arg_err::<()>(
                "invalid_arg".to_string(),
                None,
                None
            )
            .unwrap_err()
            .error_type(),
            ErrorType::InvalidArgumentError
        );
    }
}
