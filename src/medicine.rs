use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use chrono::{Local, NaiveTime};
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

#[derive(Debug)]
pub struct MedicineTimer {
    medicines: HashMap<Medicine, (bool, NaiveTime)>,
}


impl MedicineTimer {
    pub fn new() -> Self {
        let mut medicines = HashMap::new();
        let current_time = Local::now().time();

        for &medicine in &[
            Medicine::Cephalexin,
            Medicine::Oxycodone,
            Medicine::Ibuprofen,
            Medicine::Lorazepam,
            Medicine::Allegra,
        ] {
            medicines.insert(medicine, (false, current_time));
        }

        MedicineTimer { medicines }
    }

    pub fn toggle(&mut self, medicine: &Medicine) {
        if let Some(med) = self.medicines.get_mut(medicine) {
            let (status, last_toggled_time) = med;
            if !*status {
                *last_toggled_time = Local::now().time();
            }
            *status = !*status;
        }
    }

    pub fn check(&self, medicine: &Medicine) -> bool {
        if let Some(&(status, _)) = self.medicines.get(medicine) {
            status
        } else {
            false
        }
    }

    pub fn get_field(&self, medicine: &Medicine) -> (bool, NaiveTime) {
        if let Some(&field) = self.medicines.get(&medicine) {
            field
        } else {
            (false, Local::now().time())
        }
    }

    pub fn set_time(&mut self, medicine: &Medicine, new_time: NaiveTime) {
        if let Some(med) = self.medicines.get_mut(medicine) {
            let (_, last_toggled_time) = med;
            *last_toggled_time = new_time;
        }
    }
    pub fn set_toggle(&mut self, medicine: &Medicine, flag: bool) {
        if let Some(med) = self.medicines.get_mut(medicine) {
            let (toggle, _) = med;
            *toggle = flag;
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
