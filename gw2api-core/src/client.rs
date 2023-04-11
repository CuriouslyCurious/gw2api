use minreq::Response;

use std::borrow::Cow::{self, Borrowed, Owned};
use std::fmt::{self, Display};

use crate::endpoint::Endpoint;
use crate::error::{ApiError, ApiErrorKind};

// Base url to the GW2 API.
pub const BASE_URL: &str = "https://api.guildwars2.com";
// Wait max 10 seconds for a response from the server.
pub const TIMEOUT: u64 = 10;

/// All available localisations that are supported by the official Guild Wars 2 API. Default is
/// `English`.
#[derive(Debug, PartialEq)]
pub enum Localisation {
    English,
    Spanish,
    German,
    French,
    Chinese,
}

/// Converts the `Localisation` to a valid localisation suffix `String` for the Guild Wars 2 API.
impl Display for Localisation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::English => "en",
            Self::Spanish => "es",
            Self::German => "de",
            Self::French => "fr",
            Self::Chinese => "zh",
        };
        f.write_str(str)
    }
}

impl Default for Localisation {
    fn default() -> Self {
        Localisation::English
    }
}

/// Client that performs requests to the API
#[derive(Debug, Default)]
pub struct Client<'a> {
    /// The API key used for endpoints that require authentication.
    api_key: Option<String>,
    /// The language that the response will be in. Defaults to English if left empty as per the
    /// official Guild Wars 2 API behvaiour.
    lang: Localisation,
    /// Base url of the API.
    base_url: Cow<'a, str>,
    /// Timeout for requests (defaults to [`TIMEOUT`])
    timeout: u64,
}

/// A builder for creating endpoint requests, this is only used internally in the library.
pub struct RequestBuilder<'a> {
    /// URI of the endpoint.
    pub uri: Cow<'a, str>,
    /// Whether or not to use authentication.
    pub uses_auth: bool,
    /// Whether or not to the endpoint is localised.
    pub is_localised: bool,
}

impl<'a> RequestBuilder<'a> {
    /// Create a new RequestBuilder to construct a new request.
    pub fn new<T>(uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            uri: uri.into(),
            uses_auth: false,
            is_localised: false,
        }
    }

    /// Set the request as requiring authentication.
    pub fn authenticated(mut self, value: bool) -> Self {
        self.uses_auth = value;
        self
    }

    /// Set the request as being able to be localised.
    pub fn localised(mut self, value: bool) -> Self {
        self.is_localised = value;
        self
    }
}

/// Alias for `Result<T, ApiError>`.
pub type Result<T> = std::result::Result<T, ApiError>;

impl<'a> Client<'a> {
    /// Creates a new `Client` to interface with the Guild Wars 2 API.
    pub fn new() -> Client<'a> {
        Client {
            api_key: None,
            lang: Localisation::default(),
            base_url: Borrowed(BASE_URL),
            timeout: TIMEOUT,
        }
    }

    /// Sets the API key of the client with a valid Guild Wars 2 API key.
    pub fn set_api_key(&mut self, api_key: String) -> &mut Self {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the language to be used in responses, applies to item names and what not.
    pub fn set_lang(&mut self, lang: Localisation) -> &mut Self {
        self.lang = lang;
        self
    }

    /// Sets the base url for the API.
    pub fn set_base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = Owned(base_url);
        self
    }

    /// Sets the timeout for the HTTP requests.
    pub fn set_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Request an endpoint.
    pub fn request<T>(&self, builder: RequestBuilder) -> Result<T>
    where
        T: Endpoint,
    {
        let request = self.create_request(builder)?;
        let response = request.send()?;
        Client::handle_response(response)
    }

    /// Creates a request object, setting up headers and timeout, for a given URL.
    pub(crate) fn create_request(&self, builder: RequestBuilder) -> Result<minreq::Request> {
        let full_url = format!("{}/{}", self.base_url, builder.uri);
        let mut request = minreq::get(full_url).with_timeout(TIMEOUT);

        // Only include the localization header if necessary.
        if builder.is_localised {
            request = request.with_header("Accept-Language", self.lang.to_string());
        }

        if builder.uses_auth {
            let authorization_value = match self.api_key.as_ref() {
                Some(key) => format!("Bearer {}", key),
                None => return Err(ApiError::new(ApiErrorKind::ApiKeyNotSet)),
            };
            request = request.with_header("Authorization", authorization_value);
        }

        Ok(request)
    }

    /// Handles the initial response of a request by looking at the status codes.
    ///
    /// Returns the deserialized type or raises an `ApiError` upon a receiving an error,
    /// respectively.
    pub(crate) fn handle_response<T>(response: Response) -> Result<T>
    where
        T: Endpoint,
    {
        match response.status_code {
            // Ok
            200 => Ok(response.json::<T>()?),
            // Partial
            206 => Ok(response.json::<T>()?),
            // Forbidden
            403 => Err(ApiError::new(ApiErrorKind::Forbidden)),
            // Not Found
            404 => Err(ApiError::new(ApiErrorKind::NotFound)),
            // Timeout
            408 => Err(ApiError::new(ApiErrorKind::Timeout)),
            // Endpoint is disabled
            503 => Err(ApiError::new(ApiErrorKind::EndpointDisabled)),
            // Misc.
            s => Err(ApiError::new(ApiErrorKind::Custom(s.to_string()))),
        }
    }

    /// Returns an `Option` containing a string slice of the Guild Wars 2 API key for the
    /// Client object if it exists, otherwise None is returned in the Option.
    pub fn api_key(&self) -> Option<&String> {
        self.api_key.as_ref()
    }

    /// Returns a reference to the `Localisation` enum object.
    pub fn lang(&self) -> &Localisation {
        &self.lang
    }

    /// Returns the base url.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //TODO: Make tests for:
    //  * timeout
    //  * requesting any endpoint
    //  * errors - parsing and otherwise

    #[test]
    fn create_client() {
        let api_key =
            "ABCDEFGH-1324-5678-9012-IJKLMNOPQRSTUVXYZABC-1234-5678-9012-ABCDEFGHIJKL".to_string();
        let mut client = Client::new();
        client
            .set_api_key(api_key.clone())
            .set_lang(Localisation::French);
        assert_eq!(&api_key, client.api_key().unwrap());
        assert_eq!(&Localisation::French, client.lang());
    }

    #[test]
    fn create_request_builder() {
        let uri = "/v0/not_a_valid_uri";
        let builder = RequestBuilder::new(uri).localised(true).authenticated(true);
        assert_eq!(builder.uri, uri);
        assert_eq!(builder.uses_auth, true);
        assert_eq!(builder.is_localised, true);
    }

    #[test]
    fn create_request() {
        let client = Client::new();
        let builder = RequestBuilder::new("/v0/not_a_valid_uri")
            .localised(true)
            .authenticated(true);
        let request = client.create_request(builder);
        assert_eq!(request.is_ok(), true);
    }

    #[test]
    fn handle_response() {}
}
