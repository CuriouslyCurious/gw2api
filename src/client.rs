use minreq::Response;
use serde::de::DeserializeOwned;

use std::fmt::{self, Display};

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
pub struct Client {
    /// The API key used for endpoints that require authentication.
    api_key: Option<String>,
    /// The language that the response will be in. Defaults to English if left empty as per the
    /// official Guild Wars 2 API behvaiour.
    lang: Localisation,
}

impl Client {
    /// Creates a new `Client` to interface with the Guild Wars 2 API.
    pub fn new() -> Client {
        Client {
            api_key: None,
            lang: Localisation::default(),
        }
    }

    /// Sets the API key of the client with a valid Guild Wars 2 API key.
    pub fn set_api_key(mut self, api_key: String) -> Client {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the language to be used in responses, applies to item names and what not.
        pub fn set_lang(mut self, lang: Localisation) -> Client {
        self.lang = lang;
        self
    }

    // /// Creates a language HTTP header from the client's given language, if no language is given it
    // /// will default to English.
    // fn create_lang_header(&self) -> Header {
    //     let lang = self.lang().unwrap_or(&Localisation::English).to_string();
    //     Header::new("Accept-Language", &lang)
    // }
    //
    // /// Creates a HTTP authorization header from the client's given API key, if no key is set it
    // /// will panic.
    // fn create_auth_header(&self) -> Header {
    //     let api_key = self.api_key().expect("Guild Wars 2 API key is not set").to_owned();
    //     Header::new("Authorization", &format!("Bearer {}", api_key))
    // }

    /// Make a request to the Guild Wars 2 API with the given url (which has to include version)
    /// as endpoint.
    pub fn request<T>(&self, url: &str) -> Result<T, ApiError>
    where T: DeserializeOwned {
        let full_url = format!("{base_url}/{url}", base_url=BASE_URL, url=url);
        //let lang_header = self.create_lang_header();
        let response = minreq::get(&full_url)
            .with_header("Accept-Language", self.lang.to_string())
            //.set(lang_header.name(), lang_header.value())
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
        let full_url = format!("{base_url}/{url}", base_url=BASE_URL, url=url);

        let response = minreq::get(&full_url)
            .with_header("Accept-Language", self.lang.to_string())
            // TODO: Make this into an error
            .with_header("Authorization", format!("Bearer {}", self.api_key.as_ref().expect("Guild Wars 2 API key is not set")))
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
            200 => Ok(response.json::<T>().unwrap()),
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
    pub fn api_key(&self) -> Option<&str> {
        match &self.api_key {
            Some(key) => Some(&key),
            None => None,
        }
    }

    /// Returns a reference to the `Localisation` enum object.
    pub fn lang(&self) -> &Localisation {
        &self.lang
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
