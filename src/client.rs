use ureq::{Header, Response};
use crate::error::ApiError;
use serde::de::DeserializeOwned;

// Base url to the GW2 API.
pub const BASE_URL: &str = "https://api.guildwars2.com";
// Wait max 10 seconds for a response from the server.
pub const TIMEOUT: u64 = 10_000;

/// All available localisations that are supported by the official Guild Wars 2 API.
#[derive(Debug, PartialEq)]
pub enum Localisation {
    English,
    Spanish,
    German,
    French,
    Chinese,
}

impl ToString for Localisation {
    /// Converts the `Localisation` to a valid localisation suffix `String` for the Guild Wars 2 API.
    fn to_string(&self) -> String {
        match self {
            Localisation::English => "en".to_string(),
            Localisation::Spanish => "es".to_string(),
            Localisation::German => "de".to_string(),
            Localisation::French => "fr".to_string(),
            Localisation::Chinese => "zh".to_string(),
        }
    }
}

/// Client that performs requests to the API
pub struct Client {
    /// The API key used for endpoints that require authentication.
    api_key: Option<String>,
    /// The language that the response will be in. Defaults to English if left empty as per the
    /// official Guild Wars 2 API behvaiour.
    lang: Option<Localisation>,
}

impl Client {
    /// Creates a new `Client` to interface with the Guild Wars 2 API.
    pub fn new() -> Client {
        Client {
            api_key: None,
            lang: None,
        }
    }

    /// Sets the API key of the client with a valid Guild Wars 2 API key.
    pub fn set_api_key(mut self, api_key: String) -> Client {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the language to be used in responses, applies to item names and what not.
        pub fn set_lang(mut self, lang: Localisation) -> Client {
        self.lang = Some(lang);
        self
    }

    /// Creates a language HTTP header from the client's given language, if no language is given it
    /// will default to English.
    fn create_lang_header(&self) -> Header {
        let lang = self.lang().unwrap_or(&Localisation::English).to_string();
        let header = Header::new("Accept-Language", &lang);
        header
    }

    /// Creates a HTTP authorization header from the client's given API key, if no key is set it
    /// will panic.
    fn create_auth_header(&self) -> Header {
        let api_key = self.api_key().expect("Guild Wars 2 API key is not set").to_owned();
        let header = Header::new("Authorization", &format!("Bearer {}", api_key));
        header
    }

    /// Make a request to the Guild Wars 2 API with the given url (which has to include version)
    /// as endpoint.
    pub fn request<T>(&self, url: &str) -> Result<T, ApiError>
    where T: DeserializeOwned {
        let full_url = format!("{base_url}/{url}", base_url=BASE_URL, url=url);
        let lang_header = self.create_lang_header();
        let response = ureq::get(&full_url)
            .set(lang_header.name(), lang_header.value())
            .timeout_connect(TIMEOUT)
            .timeout_read(TIMEOUT)
            .call();
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
        let lang_header = self.create_lang_header();
        let auth_header = self.create_auth_header();

        let response = ureq::get(&full_url)
            .set(lang_header.name(), lang_header.value())
            .set(auth_header.name(), auth_header.value())
            .timeout_connect(TIMEOUT)
            .timeout_read(TIMEOUT)
            .call();
        Client::handle_response(response)
    }

    /// Handles the initial response of a request by looking at the status codes or if the request
    /// timed out. Returns the deserialized type or raises an `ApiError` upon a receiving an error,
    /// respectively.
    fn handle_response<T>(response: Response) -> Result<T, ApiError>
    where T: DeserializeOwned {
        if response.ok() {
            return Ok(response.into_json_deserialize::<T>().unwrap());
        } else {
            match response.status() {
                // Forbidden
                403 => Err(ApiError::new(response.into_json().unwrap())),
                // Not Found
                404 => Err(ApiError::new(response.into_json().unwrap())),
                // Timeout
                408 =>
                Err(ApiError::new("Client timed out. Probably due to the official API being down.".to_string())),
                _ => Err(ApiError::new(response.into_json().unwrap())),
            }
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

    /// Returns an `Option` to a reference of the `Localisation` enum object if given,
    /// otherwise None is returned in the Option.
    pub fn lang(&self) -> Option<&Localisation> {
        match &self.lang {
            Some(lang) => Some(&lang),
            None => None,
        }
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
        assert_eq!(&Localisation::French, client.lang().unwrap());
    }
}
