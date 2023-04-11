use log::{error, warn};
use regex::Regex;
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

impl ErrorType {
    fn default() -> Self {
        Self::UndefinedError
    }
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

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ErrorCodes {
    Code(String),
    Unmapped,
}

impl ErrorCodes {
    pub fn default() -> ErrorCodes {
        ErrorCodes::Unmapped
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MappedErrors {
    /// This field contains the error message.
    msg: String,

    /// This field contains the error type. This field is used to standardize
    /// errors codes.
    error_type: ErrorType,

    /// If dispatched error is expected or not.
    expected: bool,

    /// This field contains the error code. This field is used to standardize
    /// errors evaluation in downstream applications.
    code: ErrorCodes,
}

impl Error for MappedErrors {}

impl Display for MappedErrors {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let code_key = MappedErrors::code_key();
        let error_type_key = MappedErrors::error_type_key();

        let code_value = match self.code.to_owned() {
            ErrorCodes::Code(code) => code,
            ErrorCodes::Unmapped => String::from("none"),
        };

        write!(
            f,
            "[{}={},{}={}] {}",
            code_key, code_value, error_type_key, self.error_type, self.msg
        )
    }
}

impl MappedErrors {
    // ? -----------------------------------------------------------------------
    // ? INSTANCE METHODS
    //
    // Getters
    //
    // ? -----------------------------------------------------------------------

    /// This method returns the error type of the current error.
    pub fn error_type(&self) -> ErrorType {
        self.error_type
    }

    /// This method returns the error message of the current error.
    pub fn msg(&self) -> String {
        self.msg.to_owned()
    }

    /// This method returns the error code key of the current error.
    pub fn code(&self) -> ErrorCodes {
        self.code.to_owned()
    }

    // ? -----------------------------------------------------------------------
    // ? INSTANCE METHODS
    //
    // Modifiers
    //
    // ? -----------------------------------------------------------------------

    /// Evoked when a Err return is desired.
    pub fn as_error(self) -> Result<(), Self> {
        Err(self)
    }

    /// Dispatches an log error indicating expected error.
    pub fn with_exp_true(self) -> Self {
        warn!("{:?}", &self.to_string());
        self
    }

    /// Dispatches an log error indicating unexpected error.
    pub fn with_exp_false(self) -> Self {
        error!("Unexpected error: {}", &self.to_string());
        self
    }

    /// Set the error code of the current error.
    pub fn with_code(mut self, code: String) -> Self {
        if code == "none" {
            self.code = ErrorCodes::Unmapped;
            return self;
        }

        self.code = ErrorCodes::Code(code);
        self
    }

    pub fn with_previous(mut self, prev: MappedErrors) -> Self {
        self.msg = format!(
            "[CURRENT_ERROR] {:?}\n[PRECEDING_ERROR] {:?}",
            self.msg, &prev.msg
        );

        self
    }

    pub fn with_error_type(mut self, error_type: ErrorType) -> Self {
        self.error_type = error_type;
        self
    }

    // ? -----------------------------------------------------------------------
    // ? STRUCTURAL METHODS
    // ? -----------------------------------------------------------------------

    /// Build a anemic MappedError instance.
    pub(super) fn default(msg: String) -> Self {
        Self {
            msg,
            error_type: ErrorType::default(),
            expected: false,
            code: ErrorCodes::default(),
        }
    }

    /// This method returns a new `MappedErrors` struct.
    pub(super) fn new(
        msg: String,
        exp: Option<bool>,
        prev: Option<MappedErrors>,
        error_type: ErrorType,
    ) -> Self {
        let exp = exp.unwrap_or(true);

        if !exp {
            error!("Unexpected error: ({}){}", &error_type, &msg);
        } else {
            warn!("{:?}", &msg);
        }

        if prev.is_some() {
            let updated_msg = format!(
                "[Current error] {:?}; [Previous error] {:?}",
                msg,
                &prev.unwrap().msg
            );

            return Self::new(updated_msg, Some(exp), None, error_type);
        }

        Self {
            msg,
            error_type,
            expected: exp,
            code: ErrorCodes::default(),
        }
    }

    /// Set the error type of the current error.
    pub(self) fn code_key() -> &'static str {
        "code"
    }

    /// Set the error type of the current error.
    pub(self) fn error_type_key() -> &'static str {
        "error_type"
    }

    /// This method returns a new `MappedErrors` struct from a string.
    pub fn from_str_msg(msg: String) -> Self {
        let pattern = Regex::new(
            r"^\[code=([a-zA-Z0-9]+),error_type=([a-zA-Z-]+)\]\s(.+)$",
        )
        .unwrap();

        if pattern.is_match(&msg) {
            let capture = pattern.captures(&msg).unwrap();
            let code = capture[1].to_string();
            let msg = capture[3].to_string();

            let error_type = match ErrorType::from_str(&capture[2]) {
                Ok(error_type) => error_type,
                Err(_) => ErrorType::UndefinedError,
            };

            return MappedErrors::new(msg, None, None, error_type)
                .with_code(code);
        };

        MappedErrors::new(msg, None, None, ErrorType::UndefinedError)
    }
}

// * ---------------------------------------------------------------------------
// * TESTS
// * ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    #[test]
    fn test_error_type() {
        fn error_dispatcher() -> Result<(), super::MappedErrors> {
            Err(super::MappedErrors::new(
                "This is a test error".to_string(),
                Some(true),
                None,
                super::ErrorType::UndefinedError,
            ))
        }

        fn error_handler() -> Result<(), super::MappedErrors> {
            error_dispatcher()?;
            Ok(())
        }

        let response = error_handler().unwrap_err();

        assert_eq!(response.error_type(), super::ErrorType::UndefinedError);
    }

    #[test]
    fn test_error_msg() {
        fn error_dispatcher() -> Result<(), super::MappedErrors> {
            Err(super::MappedErrors::new(
                "This is a test error".to_string(),
                Some(true),
                None,
                super::ErrorType::UndefinedError,
            ))
        }

        fn error_handler() -> Result<(), super::MappedErrors> {
            error_dispatcher()?;
            Ok(())
        }

        let response = error_handler().unwrap_err();

        assert_eq!(
            response.to_string(),
            "[code=none,error_type=undefined-error] This is a test error"
        );
    }

    #[test]
    fn test_from_msg() {
        let msg = "[code=none,error_type=undefined-error] This is a test error";

        let response = super::MappedErrors::from_str_msg(msg.to_string());

        assert_eq!(response.to_string(), msg);
    }
}
