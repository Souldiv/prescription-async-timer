use std::collections::HashMap;

use chrono::offset::TimeZone;
use chrono::{DateTime, Local, NaiveTime};

use crate::medicine::{Medicine, Timer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
struct LocalDateTime(DateTime<Local>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    medicine: Medicine,
    taken_at: LocalDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn from_actions(list_of_actions: &Vec<Action>) -> Config {
        let new_conf = list_of_actions.into_iter().fold(Config::new(), |mut acc, act| {
            *acc.conf.entry(act.medicine).or_insert(0) += 1;
            acc
        });
        return new_conf;
    }

    fn calculate_remaining_time(&self, medicine: &Medicine, taken_at: &NaiveTime) -> u64{
        let seconds = self.get_default_duration(medicine);

        // calculate duration and create NaiveTime Object
        let duration = chrono::Duration::seconds(seconds as i64);
        // projected end time of the timer
        let end_time = *taken_at + duration;
        
        let mut remaining_time = 0;
        // calculate remaining time of the timer
        if end_time > Local::now().time() {
            remaining_time = end_time.signed_duration_since(Local::now().time()).num_seconds();
        }

        return remaining_time as u64;

    }

    fn format_seconds(&self, remaining_time: u64) -> String {
        let hours = remaining_time / 3600;
        let minutes = (remaining_time % 3600) / 60;
        let seconds = remaining_time % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn calculate_all_remaining(&self, timer: Timer) -> String{
        let mut elapsed_times = Vec::new();
        
        let t = timer.lock().unwrap();

        if t.get_field(Medicine::Cephalexin).0 {
            let rm_tm = self.calculate_remaining_time(&Medicine::Cephalexin, &t.get_field(Medicine::Cephalexin).1);
            elapsed_times.push(("Cephalexin", self.format_seconds(rm_tm)));
        }
        if  t.get_field(Medicine::Oxycodone).0 {
            let rm_tm = self.calculate_remaining_time(&Medicine::Oxycodone, &t.get_field(Medicine::Oxycodone).1);
            elapsed_times.push(("Oxycodone", self.format_seconds(rm_tm)));
        }
        if  t.get_field(Medicine::Ibuprofen).0 {
            let rm_tm = self.calculate_remaining_time(&Medicine::Ibuprofen, &t.get_field(Medicine::Ibuprofen).1);
            elapsed_times.push(("Ibuprofen",  self.format_seconds(rm_tm)));
        }
        if  t.get_field(Medicine::Lorazepam).0 {
            let rm_tm = self.calculate_remaining_time(&Medicine::Lorazepam, &t.get_field(Medicine::Lorazepam).1);
            elapsed_times.push(("Lorazepam",  self.format_seconds(rm_tm)));
        }
        if  t.get_field(Medicine::Allegra).0 {
            let rm_tm = self.calculate_remaining_time(&Medicine::Allegra, &t.get_field(Medicine::Allegra).1);
            elapsed_times.push(("Allegra",  self.format_seconds(rm_tm)));
        }

        let formatted_times: Vec<String> = elapsed_times
            .iter()
            .map(|(name, time)| format!("{}: {}", name, time))
            .collect();

        formatted_times.join(", ")
    }

    pub fn create_timer_durations(&self, list_of_actions: &Vec<Action>) -> HashMap<Medicine, u64> {
        let mut last_action: HashMap<Medicine, NaiveTime> = HashMap::new();
        let mut result: HashMap<Medicine, u64> = HashMap::new();
        // get the last medicine taken and it's time
        for action in list_of_actions {
            last_action.insert(action.medicine, action.taken_at.0.time());
        }

        for (medicine, last_taken_at) in last_action {
            result.insert(medicine, self.calculate_remaining_time(&medicine, &last_taken_at));
            }
        
        return result;
        }

    pub fn get_default_duration(&self, med: &Medicine) -> u64 {
        // values are in seconds
        return match med {
            Medicine::Cephalexin => 30,
            Medicine::Oxycodone => 120,
            Medicine::Ibuprofen => 30,
            Medicine::Lorazepam => 30,
            Medicine::Allegra => 30
        };
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
