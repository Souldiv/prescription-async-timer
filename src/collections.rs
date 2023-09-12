use std::collections::HashMap;

use chrono::offset::TimeZone;
use chrono::{DateTime, Local, NaiveTime};

use crate::medicine::{Medicine, Timer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
struct LocalDateTime(DateTime<Local>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    medicine: Medicine,
    taken_at: LocalDateTime,
}

#[derive(Clone)]
struct MedicineConfig {
    max_limit: usize,
    default_duration: u64,
}

#[derive(Clone)]
pub struct Config {
    medicines: HashMap<Medicine, MedicineConfig>,
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
    pub fn new() -> Self {
        let mut medicines = HashMap::new();
        let mut conf = HashMap::new();

        // Configure medicines
        medicines.insert(
            Medicine::Cephalexin,
            MedicineConfig {
                max_limit: 4,
                default_duration: 30,
            },
        );
        medicines.insert(
            Medicine::Oxycodone,
            MedicineConfig {
                max_limit: usize::MAX,
                default_duration: 120,
            },
        );
        medicines.insert(
            Medicine::Ibuprofen,
            MedicineConfig {
                max_limit: 4,
                default_duration: 30,
            },
        );
        medicines.insert(
            Medicine::Lorazepam,
            MedicineConfig {
                max_limit: 1,
                default_duration: 30,
            },
        );
        medicines.insert(
            Medicine::Allegra,
            MedicineConfig {
                max_limit: 1,
                default_duration: 30,
            },
        );

        // Initialize conf HashMap
        for &medicine in &[
            Medicine::Cephalexin,
            Medicine::Oxycodone,
            Medicine::Ibuprofen,
            Medicine::Lorazepam,
            Medicine::Allegra,
        ] {
            conf.insert(medicine, 0);
        }

        Config { medicines, conf }
    }

    pub fn from_actions(list_of_actions: &[Action]) -> Self {
        let mut config = Config::new();

        for action in list_of_actions {
            config.increment_count(&action.medicine);
        }

        config
    }

    fn increment_count(&mut self, medicine: &Medicine) {
        if let Some(val) = self.conf.get_mut(medicine) {
            let max_limit = self.medicines[medicine].max_limit;

            if *val + 1 <= max_limit {
                *val += 1;
            }
        }
    }

    fn calculate_remaining_time(&self, medicine: &Medicine, taken_at: &NaiveTime) -> u64 {
        let seconds = self.medicines[medicine].default_duration;

        // calculate duration and create NaiveTime Object
        let duration = chrono::Duration::seconds(seconds as i64);

        println!("taken at {:?} duration {:?}", taken_at, duration);
        // projected end time of the timer
        let end_time = *taken_at + duration;

        println!("end_time {:?}", end_time);

        let mut remaining_time = 0;
        // calculate remaining time of the timer
        if end_time > Local::now().time() {
            remaining_time = end_time
                .signed_duration_since(Local::now().time())
                .num_seconds();
        }

        println!("remaining time {:?}", remaining_time);

        return remaining_time as u64;
    }

    fn format_seconds(&self, remaining_time: u64) -> String {
        let hours = remaining_time / 3600;
        let minutes = (remaining_time % 3600) / 60;
        let seconds = remaining_time % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn calculate_all_remaining(&self, timer: Timer) -> String {
        let mut elapsed_times = Vec::new();

        let t = timer.lock().unwrap(); // Lock the timer

        for &medicine in &[
            Medicine::Cephalexin,
            Medicine::Oxycodone,
            Medicine::Ibuprofen,
            Medicine::Lorazepam,
            Medicine::Allegra,
        ] {
            if t.check(&medicine) {
                let rm_tm = self.calculate_remaining_time(
                    &medicine,
                    &(t.get_field(&medicine).1),
                );
                elapsed_times.push((medicine, rm_tm));
            }
        }

        // Release the lock on the timer
        drop(t);

        let formatted_times: Vec<String> = elapsed_times
            .iter()
            .map(|&(medicine, time)| format!("{}: {}", medicine, self.format_seconds(time)))
            .collect();

        formatted_times.join(", ")
    }

    pub fn check_and_insert(&mut self, med: &Medicine) -> bool {
        if let Some(val) = self.conf.get_mut(med) {
            let config = &self.medicines[med];
            let limit = config.max_limit;

            if *val + 1 <= limit {
                *val += 1;
                return true;
            } else {
                return false;
            }
        }
        false
    }

    pub fn create_timer_durations(&self, list_of_actions: &Vec<Action>) -> HashMap<Medicine, u64> {
        let mut last_action: HashMap<Medicine, NaiveTime> = HashMap::new();
        let mut result: HashMap<Medicine, u64> = HashMap::new();
        // get the last medicine taken and it's time
        for action in list_of_actions {
            last_action.insert(action.medicine, action.taken_at.0.time());
        }

        for (medicine, last_taken_at) in last_action {
            result.insert(
                medicine,
                self.calculate_remaining_time(&medicine, &last_taken_at),
            );
        }

        return result;
    }

    pub fn get_default_duration(&self, med: &Medicine) -> u64 {
        match self.medicines.get(med) {
            Some(config) => config.default_duration,
            None => 0, // or any other default value you prefer for unconfigured medicines
        }
    }

}
