use std::fmt::{self, Display};
use std::error::Error;

/// This error is raised whenever an error occurs when calling the Guild Wars 2 API, for example by
/// trying to access a resource that requires authentication without a valid API key, or trying to
/// access a non-existent item id.
///
/// The most relevant field is the text field describing the error, rest are probably for internal
/// use by ArenaNet.
#[derive(Debug)]
pub struct ApiError {
    /// Errorkind
    err: Box<ApiErrorKind>,
}

// impl de::Error for ApiError {
//     fn custom<T: std::string::ToString>(msg: T) -> Self {
//         ApiError {
//             text: msg.to_string(),
//         }
//     }
// }

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.err)
    }
}

impl From<minreq::Error> for ApiError {
    fn from(err: minreq::Error) -> Self {
        Self {
            err: Box::new(ApiErrorKind::ClientError(err))
        }
    }
}

impl ApiError {
    /// Create a new ApiError from any type T that implements the Display trait.
    pub fn new(err: ApiErrorKind) -> ApiError {
        Self {
            err: Box::new(err),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ApiErrorKind {
    ClientError(minreq::Error),
    ApiTimeout,
    Forbidden,
    NotFound,
    ApiKeyNotSet,
    Custom(String),
    //DeserializeError(&dyn de::Error),
}

impl Error for ApiErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ClientError(e) => Some(e),
            Self::ApiTimeout => None,
            Self::Forbidden => None,
            Self::NotFound => None,
            Self::ApiKeyNotSet => None,
            Self::Custom(..) => None,
        }
    }
}

impl Display for ApiErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientError(e) => f.write_str(&e.to_string()),
            Self::ApiTimeout => f.write_str("Client timed out. Check your internet connection or the status of the official API."),
            Self::Forbidden => f.write_str("Unable to access resource. You probably lack the appropriate premissions on your GW2 API key to access this resource."),
            Self::NotFound => f.write_str("Unable to find the endpoint."),
            Self::ApiKeyNotSet => f.write_str("GW2 API key not set while trying to access resource that needs one."),
            Self::Custom(s) => f.write_str(s),
            //Self::DeserializeError(e) => f.write(&e.to_string()),
        }
    }
}
