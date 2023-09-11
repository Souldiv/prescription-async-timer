use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Medicine {
    Cephalexin,
    Oxycodone,
    Ibuprofen,
    Lorazepam,
    Allegra,
}

impl fmt::Display for Medicine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Medicine::Cephalexin => write!(f, "Cephalexin"),
            Medicine::Oxycodone => write!(f, "Oxycodone"),
            Medicine::Ibuprofen => write!(f, "Ibuprofen"),
            Medicine::Lorazepam => write!(f, "Lorazepam"),
            Medicine::Allegra => write!(f, "Allegra"),
        }
    }
}

impl<'de> Deserialize<'de> for Medicine {
    fn deserialize<D>(deserializer: D) -> Result<Medicine, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a string from the input
        let s: String = Deserialize::deserialize(deserializer)?;

        // Map the input string to the corresponding enum variant
        match s.as_str() {
            "Cephalexin" => Ok(Medicine::Cephalexin),
            "Oxycodone" => Ok(Medicine::Oxycodone),
            "Ibuprofen" => Ok(Medicine::Ibuprofen),
            "Lorazepam" => Ok(Medicine::Lorazepam),
            "Allegra" => Ok(Medicine::Allegra),
            _ => Err(serde::de::Error::custom(format!("Unknown medicine: {}", s))),
        }
    }
}

impl Serialize for Medicine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}
