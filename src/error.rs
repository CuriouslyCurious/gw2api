use serde::de;
use std::fmt::{self, Display};

/// This error is raised whenever an error occurs when calling the Guild Wars 2 API, for example by
/// trying to access a resource that requires authentication without a valid API key, or trying to
/// access a non-existent item id.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ApiError {
    /// Text describing the error retrieved from the API.
    text: String,
}

impl de::Error for ApiError {
    fn custom<T: Display>(msg: T) -> Self {
        ApiError {
            text: msg.to_string(),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
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
