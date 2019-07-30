use crate::client::Client;

/// Contains a Guild Wars 2 build version id
#[derive(Debug, Deserialize)]
pub struct Build {
    /// The current build version id
    id: u32,
}

impl Build {
    /// Returns a Build struct containing the current build version within the `id` field.
    pub fn get_build(client: &Client) -> Build {
        client.request("/v2/build").unwrap().json().unwrap()
    }

    /// Returns the id of the current Guild Wars 2 build version
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::api::build::Build;
    #[test]
    fn get_build() {
        let client = Client::new();
        assert_ne!(Build::get_build(&client).id(), 0)
    }
}
