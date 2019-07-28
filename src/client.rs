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
    /// Converts the `Localisation` to a valid localisation suffix `String` for the Guild Wars 2 API
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
    /// The API Key used for endpoints that require authentication
    api_key: Option<String>,
    /// The language that the response will be in. Defaults to English if left empty as per the
    /// offical Guild Wars 2 API behvaiour.
    lang: Option<Localisation>,
    /// Reqwest client that actually handles any request.
    client: reqwest::Client,
}

impl Client {
    /// Creates a new API client with an optional language defined and/or an optional valid
    /// Guild Wars 2 API key
    // XXX: Might be more suitable using the Builder design pattern: https://doc.rust-lang.org/cargo/reference/manifest.html
    pub fn new(api_key: Option<String>, lang: Option<Localisation>) -> Client {
        Client {
            api_key,
            lang,
            client: reqwest::Client::new()
        }
    }

    /// Make a request to the Guild Wars 2 API with the given url (which has to include version)
    /// as endpoint.
    pub fn request(&self, url: &str) -> reqwest::Result<reqwest::Response> {
        let full_url = format!("{base_url}/{url}", base_url=BASE_URL, url=url);

        // Defaults to English if no language is specified
        let lang = self.lang().unwrap_or(&Localisation::English).to_string();

        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT_LANGUAGE, lang.parse().unwrap());
        self.client.get(&full_url).headers(headers).send()
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

    /// Returns a reference to the underlying Reqwest client
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use crate::client::*;

    #[test]
    fn create_client() {
        Client::new(None, None);
    }

    #[test]
    fn create_client_with_localisation() {
        let client = Client::new(None, Some(Localisation::French));
        assert_eq!(&Localisation::French, client.lang().unwrap());
    }

    #[test]
    fn create_client_with_api_key() {
        let api_key = "ABCDEFGH-1324-5678-9012-IJKLMNOPQRSTUVXYZABC-1234-5678-9012-ABCDEFGHIJKL"
            .to_string();
        let client = Client::new(Some(api_key.clone()), None);
        assert_eq!(&api_key, client.api_key().unwrap());
    }

}
