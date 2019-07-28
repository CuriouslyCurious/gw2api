use reqwest;

/// All available localisations that are supported by the official Guild Wars 2 API.
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
    pub fn new(lang: Option<Localisation>, api_key: Option<String>) -> Client {
        Client {
            api_key,
            lang,
            client: reqwest::Client::new()
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
}
