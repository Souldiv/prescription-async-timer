use std::fmt;
use std::sync::{Mutex, Arc};

use chrono::{Utc, NaiveTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type Timer = Arc<Mutex<MedicineTimer>>;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Medicine {
    Cephalexin,
    Oxycodone,
    Ibuprofen,
    Lorazepam,
    Allegra,
}

pub struct MedicineTimer {
    cephalexin: (bool, NaiveTime),
    oxycodone: (bool, NaiveTime),
    ibuprofen: (bool, NaiveTime),
    lorazepam: (bool, NaiveTime),
    allegra: (bool, NaiveTime),
}

impl MedicineTimer {
    pub fn new() -> Self {
        MedicineTimer {
            cephalexin: (false, Utc::now().time()),
            oxycodone: (false, Utc::now().time()),
            ibuprofen: (false, Utc::now().time()),
            lorazepam: (false, Utc::now().time()),
            allegra: (false, Utc::now().time()),
        }
    }

    pub fn toggle(&mut self, medicine: &Medicine) {
        match medicine {
            Medicine::Cephalexin => {
                let (status, last_toggled_time) = &mut self.cephalexin;
                *status = !*status;
                *last_toggled_time = Utc::now().time();
            },
            Medicine::Oxycodone => {
                let (status, last_toggled_time) = &mut self.oxycodone;
                *status = !*status;
                *last_toggled_time = Utc::now().time();
            },
            Medicine::Ibuprofen => {
                let (status, last_toggled_time) = &mut self.ibuprofen;
                *status = !*status;
                *last_toggled_time = Utc::now().time();
            },
            Medicine::Lorazepam => {
                let (status, last_toggled_time) = &mut self.lorazepam;
                *status = !*status;
                *last_toggled_time = Utc::now().time();
            },
            Medicine::Allegra => {
                let (status, last_toggled_time) = &mut self.allegra;
                *status = !*status;
                *last_toggled_time = Utc::now().time();
            },
        }
    }

    pub fn check(&self, medicine: &Medicine) -> bool {
        return match medicine {
            Medicine::Cephalexin => self.cephalexin.0,
            Medicine::Oxycodone => self.oxycodone.0,
            Medicine::Ibuprofen => self.ibuprofen.0,
            Medicine::Lorazepam => self.lorazepam.0,
            Medicine::Allegra => self.allegra.0
        };
    }

    pub fn calculate_elapsed_time(&self) -> String {
        let mut elapsed_times = Vec::new();

        if self.cephalexin.0 {
            elapsed_times.push(("Cephalexin", self.calculate_elapsed_medicine(&self.cephalexin)));
        }
        if self.oxycodone.0 {
            elapsed_times.push(("Oxycodone", self.calculate_elapsed_medicine(&self.oxycodone)));
        }
        if self.ibuprofen.0 {
            elapsed_times.push(("Ibuprofen", self.calculate_elapsed_medicine(&self.ibuprofen)));
        }
        if self.lorazepam.0 {
            elapsed_times.push(("Lorazepam", self.calculate_elapsed_medicine(&self.lorazepam)));
        }
        if self.allegra.0 {
            elapsed_times.push(("Allegra", self.calculate_elapsed_medicine(&self.allegra)));
        }

        let formatted_times: Vec<String> = elapsed_times
            .iter()
            .map(|(name, time)| format!("{}: {}", name, time))
            .collect();

        formatted_times.join(", ")
    }

    fn calculate_elapsed_medicine(&self, medicine: &(bool, NaiveTime)) -> String {
        if medicine.0 {
            // Assuming the time difference is calculated in seconds
            let duration = (chrono::Utc::now().time() - medicine.1).num_seconds();
            let hours = duration / 3600;
            let minutes = (duration % 3600) / 60;
            let seconds = duration % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            "N/A".to_string()
        }
    }
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
