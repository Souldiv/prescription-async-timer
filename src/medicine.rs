use std::fmt;
use std::sync::{Mutex, Arc};

use chrono::naive::NaiveTime;
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
    Cephalexin: (bool, NaiveTime),
    Oxycodone: (bool, NaiveTime),
    Ibuprofen: (bool, NaiveTime),
    Lorazepam: (bool, NaiveTime),
    Allegra: (bool, NaiveTime),
}

impl MedicineTimer {
    pub fn new() -> Self {
        MedicineTimer {
            Cephalexin: (false, NaiveTime::from_hms(0, 0, 0)),
            Oxycodone: (false, NaiveTime::from_hms(0, 0, 0)),
            Ibuprofen: (false, NaiveTime::from_hms(0, 0, 0)),
            Lorazepam: (false, NaiveTime::from_hms(0, 0, 0)),
            Allegra: (false, NaiveTime::from_hms(0, 0, 0)),
        }
    }

    pub fn toggle(&mut self, medicine: &Medicine) {
        match medicine {
            Medicine::Cephalexin => self.toggle_medicine(&mut self.Cephalexin),
            Medicine::Oxycodone => self.toggle_medicine(&mut self.Oxycodone),
            Medicine::Ibuprofen => self.toggle_medicine(&mut self.Ibuprofen),
            Medicine::Lorazepam => self.toggle_medicine(&mut self.Lorazepam),
            Medicine::Allegra => self.toggle_medicine(&mut self.Allegra),
        }
    }

    pub fn check(&self, medicine: &Medicine) -> bool {
        return match medicine {
            Medicine::Cephalexin => self.Cephalexin.0,
            Medicine::Oxycodone => self.Oxycodone.0,
            Medicine::Ibuprofen => self.Ibuprofen.0,
            Medicine::Lorazepam => self.Lorazepam.0,
            Medicine::Allegra => self.Allegra.0
        };
    }

    fn toggle_medicine(&mut self, medicine: &mut (bool, NaiveTime)) {
        let (status, last_toggled_time) = medicine;
        *status = !*status;
        *last_toggled_time = NaiveTime::from_hms(0, 0, 0); // Reset time when toggled
    }

    pub fn calculate_elapsed_time(&self) -> String {
        let mut elapsed_times = Vec::new();

        if self.Cephalexin.0 {
            elapsed_times.push(("Cephalexin", self.calculate_elapsed_medicine(&self.Cephalexin)));
        }
        if self.Oxycodone.0 {
            elapsed_times.push(("Oxycodone", self.calculate_elapsed_medicine(&self.Oxycodone)));
        }
        if self.Ibuprofen.0 {
            elapsed_times.push(("Ibuprofen", self.calculate_elapsed_medicine(&self.Ibuprofen)));
        }
        if self.Lorazepam.0 {
            elapsed_times.push(("Lorazepam", self.calculate_elapsed_medicine(&self.Lorazepam)));
        }
        if self.Allegra.0 {
            elapsed_times.push(("Allegra", self.calculate_elapsed_medicine(&self.Allegra)));
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
