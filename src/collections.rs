use std::collections::HashMap;

use chrono::offset::TimeZone;
use chrono::{DateTime, Local};

use crate::medicine::Medicine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
struct LocalDateTime(DateTime<Local>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    medicine: Medicine,
    taken_at: LocalDateTime,
}

#[derive(Debug)]
pub struct Config {
    date: LocalDateTime,
    conf: HashMap<Medicine, usize>,
}

impl<'de> Deserialize<'de> for LocalDateTime {
    fn deserialize<D>(deserializer: D) -> Result<LocalDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        // Parse the RFC 3339 string into a DateTime<Local>
        match Local.datetime_from_str(&s, "%+") {
            Ok(parsed_datetime) => Ok(LocalDateTime(parsed_datetime)),
            Err(_) => {
                // Handle parsing error
                Err(serde::de::Error::custom("Failed to parse RFC3339 datetime"))
            }
        }
    }
}

impl Serialize for LocalDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_rfc3339().serialize(serializer)
    }
}

impl Action {
    pub fn new(medicine: Medicine) -> Action {
        Action {
            medicine: medicine,
            taken_at: LocalDateTime(Local::now()),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        return Config {
        date: LocalDateTime(Local::now()),
        conf: HashMap::from([
                (Medicine::Cephalexin, 0),
                (Medicine::Ibuprofen, 0),
                (Medicine::Oxycodone, 0),
                (Medicine::Lorazepam, 0),
                (Medicine::Allegra, 0),
            ])
        }
    }

    pub fn from_actions(list_of_actions: Vec<Action>) -> Config {
        let new_conf = list_of_actions.into_iter().fold(Config::new(), |mut acc, act| {
            *acc.conf.entry(act.medicine).or_insert(0) += 1;
            acc
        });

        return new_conf;
    }

    pub fn check_and_insert(&mut self, med: &Medicine) -> bool {
        if let Some(val) = self.conf.get_mut(med) {
            let limit: usize = match med {
                Medicine::Cephalexin => 4,
                Medicine::Ibuprofen => 4,
                Medicine::Lorazepam => 1,
                Medicine::Oxycodone => usize::MAX,
                Medicine::Allegra => 1,
            };

            if *val + 1 <= limit {
                *val += 1;
                return true;
            } else {
                return false;
            }
        }
        return false;
    }
}
