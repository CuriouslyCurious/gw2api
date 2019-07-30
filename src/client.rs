use reqwest;
use reqwest::header::HeaderMap;

pub const BASE_URL: &str = "https://api.guildwars2.com";

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
    /// The reqwest client that actually handles any request.
    client: reqwest::Client,
}

impl Client {
    /// Creates a new `Client` to interface with the Guild Wars 2 API.
    pub fn new() -> Client {
        Client {
            api_key: None,
            lang: None,
            client: reqwest::Client::new(),
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

    /// Make a request to the Guild Wars 2 API with the given url (which has to include version)
    /// as endpoint.
    pub fn request(&self, url: &str) -> reqwest::Result<reqwest::Response> {
        let full_url = format!("{base_url}/{url}", base_url=BASE_URL, url=url);
        let headers = self.create_lang_header();

        self.client.get(&full_url).headers(headers).send()
    }

    /// Make an authenticated request to the Guild Wars 2 API with the given url (which has to
    /// include version) as endpoint. This requires that the `api_key` field of the client is set,
    /// otherwise it panics.
    ///
    /// This function may fail depending on what the settings of the API key itself are, since you
    /// can limit what resources a certain key may access. In that case the function will return
    /// an error.
    pub fn authenticated_request(&self, url: &str) -> reqwest::Result<reqwest::Response> {
        let full_url = format!("{base_url}/{url}", base_url=BASE_URL, url=url);
        let mut headers = self.create_lang_header();

        // Create authorization header
        let api_key = self.api_key().expect("Guild Wars 2 API key is not set").to_owned();
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

        self.client.get(&full_url).headers(headers).send()
    }

    /// Creates a language HTTP header from the client's given language, if no language is given it
    /// will default to English.
    fn create_lang_header(&self) -> HeaderMap {
        // Defaults to English if no language is specified
        let lang = self.lang().unwrap_or(&Localisation::English).to_string();

        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT_LANGUAGE, lang.parse().unwrap());
        headers
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

    /// Returns a reference to the underlying reqwest client.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use crate::client::*;

    #[test]
    fn create_client() {
        let api_key = "ABCDEFGH-1324-5678-9012-IJKLMNOPQRSTUVXYZABC-1234-5678-9012-ABCDEFGHIJKL"
            .to_string();
        let client = Client::new().set_api_key(api_key.clone()).set_lang(Localisation::French);
        assert_eq!(&api_key, client.api_key().unwrap());
        assert_eq!(&Localisation::French, client.lang().unwrap());
    }
}
