use serde::de;
use std::fmt::{self, Display};

/// This error is raised whenever an error occurs when calling the Guild Wars 2 API, for example by
/// trying to access a resource that requires authentication without a valid API key, or trying to
/// access a non-existent item id.
///
/// The most relevant field is the text field describing the error, rest are probably for internal
/// use by ArenaNet.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ApiError {
    ///// Error code.
    //code: Option<u32>,
    ///// Product where the error concerns.
    //product: Option<u32>,
    ///// Which module the error occurred in.
    //module: Option<u32>,
    ///// At what line the error occurred.
    //line: Option<u32>,
    /// Text describing the error retrieved from the API.
    text: String,
}

impl de::Error for ApiError {
    fn custom<T: std::string::ToString>(msg: T) -> Self {
        ApiError {
            text: msg.to_string(),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.to_string())
    }
}

impl std::error::Error for ApiError {
    fn description(&self) -> &str {
        &self.text
    }
}

impl ApiError {
    /// Create a new ApiError from any type T that implements the Display trait.
    pub fn new(text: impl Display) -> ApiError {
        ApiError {
            text: text.to_string(),
        }
    }

    /// Returns the description of the error.
    pub fn description(&self) -> &str {
        &self.text
    }
}
