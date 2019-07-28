/// Contains a Guild Wars 2 build version id
#[derive(Debug, Deserialize)]
pub struct Build {
    /// The current build version id
    id: u32,
}

impl Build {
    /// Returns a Build struct containing the current build version within the `id` field.
    pub fn get_build() -> Build {
        reqwest::get("https://api.guildwars2.com/v2/build").unwrap().json().unwrap()
    }

    /// Returns the id of the current Guild Wars 2 build version
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use crate::api::build::Build;
    #[test]
    fn get_build() {
        assert_eq!(Build::get_build().id(), 97999)
    }
}
