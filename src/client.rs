use minreq::Response;
use serde::de::DeserializeOwned;

use std::fmt::{self, Display};
use std::borrow::Cow::{self, Borrowed, Owned};

use crate::error::{ApiError, ApiErrorKind};

// Base url to the GW2 API.
pub const BASE_URL: &str = "https://api.guildwars2.com";
// Wait max 10 seconds for a response from the server.
pub const TIMEOUT: u64 = 10;

/// All available localisations that are supported by the official Guild Wars 2 API.
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
        match self {
            Localisation::English => f.write_str("en"),
            Localisation::Spanish => f.write_str("es"),
            Localisation::German => f.write_str("de"),
            Localisation::French => f.write_str("fr"),
            Localisation::Chinese => f.write_str("zh"),
        }
    }
}

impl Default for Localisation {
    fn default() -> Self {
        Localisation::English
    }
}

/// Client that performs requests to the API
#[derive(Default)]
pub struct Client<'a> {
    /// The API key used for endpoints that require authentication.
    api_key: Option<String>,
    /// The language that the response will be in. Defaults to English if left empty as per the
    /// official Guild Wars 2 API behvaiour.
    lang: Localisation,
    /// Base url of the API.
    base_url: Cow<'a, str>,

}

impl<'a> Client<'a> {
    /// Creates a new `Client` to interface with the Guild Wars 2 API.
    pub fn new() -> Client<'a> {
        Client {
            api_key: None,
            lang: Localisation::default(),
            base_url: Borrowed(BASE_URL),
        }
    }

    /// Sets the API key of the client with a valid Guild Wars 2 API key.
    pub fn set_api_key(mut self, api_key: String) -> Client<'a> {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the language to be used in responses, applies to item names and what not.
    pub fn set_lang(mut self, lang: Localisation) -> Client<'a> {
        self.lang = lang;
        self
    }

    /// Sets the base url for the API.
    pub fn set_base_url(mut self, base_url: String) -> Client<'a>{
        self.base_url = Owned(base_url);
        self
    }

    /// Make a request to the Guild Wars 2 API with the given url (which has to include version)
    /// as endpoint.
    pub fn request<T>(&self, url: &str) -> Result<T, ApiError>
    where T: DeserializeOwned {
        let full_url = format!("{base_url}/{url}", base_url=self.base_url, url=url);
        let response = minreq::get(&full_url)
            .with_header("Accept-Language", self.lang.to_string())
            .with_timeout(TIMEOUT)
            .send()?;
        Client::handle_response(response)
    }

    /// Make an authenticated request to the Guild Wars 2 API with the given url (which has to
    /// include version) as endpoint. This requires that the `api_key` field of the client is set,
    /// otherwise it panics.
    ///
    /// This function may fail depending on what the settings of the API key itself are, since you
    /// can limit what resources a certain key may access. In that case the function will return
    /// an error.
    pub fn authenticated_request<T>(&self, url: &str) -> Result<T, ApiError>
    where T: DeserializeOwned {
        let full_url = format!("{base_url}/{url}", base_url=self.base_url, url=url);

        let authorization_msg = match self.api_key.as_ref() {
            Some(key) => format!("Bearer {}", key),
            None => return Err(ApiError::new(ApiErrorKind::ApiKeyNotSet)),
        };

        let response = minreq::get(&full_url)
            .with_header("Accept-Language", self.lang.to_string())
            .with_header("Authorization", authorization_msg)
            .with_timeout(TIMEOUT)
            .send()?;
        Client::handle_response(response)
    }

    /// Handles the initial response of a request by looking at the status codes or if the request
    /// timed out. Returns the deserialized type or raises an `ApiError` upon a receiving an error,
    /// respectively.
    fn handle_response<T>(response: Response) -> Result<T, ApiError>
    where T: DeserializeOwned {
        match response.status_code {
            // Ok
            200 => Ok(response.json::<T>()?),
            // Forbidden
            403 => Err(ApiError::new(ApiErrorKind::Forbidden)),
            // Not Found
            404 => Err(ApiError::new(ApiErrorKind::NotFound)),
            // Timeout
            408 =>
            Err(ApiError::new(ApiErrorKind::ApiTimeout)),
            _ => Err(ApiError::new(ApiErrorKind::Custom(String::new()))),
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
    use crate::client::*;
    //TODO: Make tests for:
    //  * timeout
    //  * requesting any endpoint
    //  * errors - parsing and otherwise

    #[test]
    fn create_client() {
        let api_key = "ABCDEFGH-1324-5678-9012-IJKLMNOPQRSTUVXYZABC-1234-5678-9012-ABCDEFGHIJKL"
            .to_string();
        let client = Client::new().set_api_key(api_key.clone()).set_lang(Localisation::French);
        assert_eq!(&api_key, client.api_key().unwrap());
        assert_eq!(&Localisation::French, client.lang());
    }
}
