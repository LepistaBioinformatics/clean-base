use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/// This enumerator are used to standardize errors codes dispatched during the
/// `MappedErrors` struct usage.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ErrorType {
    /// This error type is used when the error type is not defined. This is the
    /// default value for the `ErrorType` enum.
    ///
    /// Related: Undefined
    UndefinedError,

    /// This error type is used when a creation error occurs.
    ///
    /// Related: CRUD
    CreationError,

    /// This error type is used when an updating error occurs.
    ///
    /// Related: CRUD
    UpdatingError,

    /// This error type is used when a fetching error occurs.
    ///
    /// Related: CRUD
    FetchingError,

    /// This error type is used when a deletion error occurs.
    ///
    /// Related: CRUD
    DeletionError,

    /// This error type is used when a use case error occurs.
    ///
    /// Related: Use Case
    UseCaseError,

    /// This error type is used when an execution error occurs. This error type
    /// is used when the error is not related to a specific action.
    ///
    /// Related: Execution
    ExecutionError,

    /// This error type is used when an invalid data repository error occurs.
    ///
    /// Related: Data Repository
    InvalidRepositoryError,

    /// This error type is used when an invalid argument error occurs.
    ///
    /// Related: Argument
    InvalidArgumentError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ErrorType::UndefinedError => write!(f, "undefined-error"),
            ErrorType::CreationError => write!(f, "creation-error"),
            ErrorType::UpdatingError => write!(f, "updating-error"),
            ErrorType::FetchingError => write!(f, "fetching-error"),
            ErrorType::DeletionError => write!(f, "deletion-error"),
            ErrorType::UseCaseError => write!(f, "use-case-error"),
            ErrorType::ExecutionError => write!(f, "execution-error"),
            ErrorType::InvalidRepositoryError => {
                write!(f, "invalid-repository-error")
            }
            ErrorType::InvalidArgumentError => {
                write!(f, "invalid-argument-error")
            }
        }
    }
}

impl FromStr for ErrorType {
    type Err = ();

    fn from_str(s: &str) -> Result<ErrorType, ()> {
        match s {
            "undefined-error" => Ok(ErrorType::UndefinedError),
            "creation-error" => Ok(ErrorType::CreationError),
            "updating-error" => Ok(ErrorType::UpdatingError),
            "fetching-error" => Ok(ErrorType::FetchingError),
            "deletion-error" => Ok(ErrorType::DeletionError),
            "use-case-error" => Ok(ErrorType::UseCaseError),
            "execution-error" => Ok(ErrorType::ExecutionError),
            "invalid-repository-error" => Ok(ErrorType::InvalidRepositoryError),
            "invalid-argument-error" => Ok(ErrorType::InvalidArgumentError),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MappedErrors {
    /// This field contains the error message.
    pub msg: String,

    /// This field contains the error type. This field is used to standardize
    /// errors codes.
    error_type: ErrorType,
}

impl Error for MappedErrors {}

impl Display for MappedErrors {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "({}): {}", self.error_type, self.msg)
    }
}

impl MappedErrors {
    /// This method returns the error type of the current error.
    pub fn error_type(&self) -> ErrorType {
        self.error_type
    }

    /// This method returns the error message of the current error.
    pub fn msg(&self) -> String {
        self.to_string()
    }

    /// This method returns a new `MappedErrors` struct.
    pub(super) fn new(
        msg: String,
        exp: Option<bool>,
        prev: Option<MappedErrors>,
        error_type: ErrorType,
    ) -> MappedErrors {
        if !exp.unwrap_or(true) {
            error!("Unexpected error: ({}){}", &error_type, &msg);
        } else {
            warn!("{:?}", &msg);
        }

        if prev.is_some() {
            let updated_msg = format!(
                "[Current error] {:?}: [Previous error] {:?}",
                msg,
                &prev.unwrap().msg
            );

            return MappedErrors::new(updated_msg, exp, None, error_type);
        }

        MappedErrors { msg, error_type }
    }
}
