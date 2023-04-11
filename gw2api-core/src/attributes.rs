use serde::Deserialize;

/// The available attributes that can be returned by the API.
#[derive(Debug, PartialEq, Deserialize, Hash, Eq)]
pub enum Attribute {
    AgonyResistance,
    // Also known as Concentration
    BoonDuration,
    ConditionDamage,
    // Also known as Expertise
    ConditionDuration,
    // Also known as Ferocity
    CritDamage,
    Healing,
    Power,
    Precision,
    Toughness,
    Vitality,
}

#[cfg(test)]
mod tests {
    use crate::attributes::Attribute;
    use std::collections::HashMap;

    #[test]
    fn deserialize_attribute() {
        let json_attribute = r#"
        {
            "Vitality": 1200
        }"#;
        let mut attribute: HashMap<Attribute, i32> = HashMap::new();
        attribute.insert(Attribute::Vitality, 1200);
        assert_eq!(attribute, serde_json::from_str(json_attribute).unwrap());
    }

    #[test]
    fn deserialize_hashmap_attributes() {
        let json_attributes = r#"
        {
            "Power": 460,
            "Precision": 460,
            "Vitality": 460
        }"#;
        let mut attributes = HashMap::new();
        attributes.insert(Attribute::Power, 460);
        attributes.insert(Attribute::Precision, 460);
        attributes.insert(Attribute::Vitality, 460);

        let json_attributes: HashMap<Attribute, i32> =
            serde_json::from_str(json_attributes).unwrap();
        assert_eq!(attributes, json_attributes);
    }
}
