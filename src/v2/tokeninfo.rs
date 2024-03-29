use serde::{Deserialize, Deserializer};

use crate::client::Client;
use crate::error::ApiError;

/// Information about a supplied API key.
#[derive(Debug, Deserialize, PartialEq)]
pub struct TokenInfo {
    /// The API key that was requested.
    pub id: String,
    /// Name of the given API key. **Warning**: The value of this field is not escaped and may contain
    /// valid HTML, JavaScript, other code. Handle with care.
    pub name: String,
    /// Permissions that the API key has.
    pub permissions: Permissions,
}

impl TokenInfo {
    /// Returns a `TokenInfo` struct containing the id given, the key's name and what permissions are
    /// set for the `Client`'s key.
    pub fn get_tokeninfo(client: &Client) -> Result<TokenInfo, ApiError> {
        client.authenticated_request("/v2/tokeninfo")
    }
}

/// Permissions a Guild Wars 2 API key can have.
#[derive(Debug, PartialEq)]
pub struct Permissions {
    account: bool,
    builds: bool,
    characters: bool,
    guilds: bool,
    inventories: bool,
    progression: bool,
    pvp: bool,
    tradingpost: bool,
    unlocks: bool,
    wallet: bool,
}

impl<'de> Deserialize<'de> for Permissions {
    /// Custom deserialization, since the API returns an array of Strings that serde cannot
    /// automatically deserialize into a bunch of booleans.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let mut permissions = Permissions {
            account: false,
            builds: false,
            characters: false,
            guilds: false,
            inventories: false,
            progression: false,
            pvp: false,
            tradingpost: false,
            unlocks: false,
            wallet: false,
        };

        let vec: Vec<String> = Vec::deserialize(deserializer)?;
        for s in vec.iter() {
            match s.as_ref() {
                "account" => permissions.account = true,
                "builds" => permissions.builds = true,
                "characters" => permissions.characters = true,
                "guilds" => permissions.guilds = true,
                "inventories" => permissions.inventories = true,
                "progression" => permissions.progression = true,
                "pvp" => permissions.pvp = true,
                "tradingpost" => permissions.tradingpost = true,
                "unlocks" => permissions.unlocks = true,
                "wallet" => permissions.wallet = true,
                &_ => (),
            }
        }

        Ok(permissions)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::v2::tokeninfo::{TokenInfo, Permissions};
    use std::env;

    #[test]
    fn get_tokeninfo() {
        let api_key = env::var("GW2_TEST_KEY").expect("GW2_TEST_KEY environment variable is not set.");
        let client = Client::new().set_api_key(api_key);
        let ti = TokenInfo::get_tokeninfo(&client).unwrap();

        let permissions = Permissions {
            account: true,
            builds: true,
            characters: true,
            guilds: true,
            inventories: true,
            progression: true,
            pvp: true,
            tradingpost: true,
            unlocks: true,
            wallet: true,
        };

        assert_eq!(permissions, ti.permissions);
    }
}
