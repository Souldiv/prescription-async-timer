use crate::collections::{Action, Config};
use crate::medicine::{Medicine, Timer};

use futures::stream::StreamExt;

use chrono::Local;
use chrono::Timelike;

use mongodb::bson::doc;
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection, Database};

use tokio::time::{sleep, Duration};

extern crate colored;
use colored::*;

// duration timer function that starts the timer
pub async fn async_sleep(med: Medicine, timer: Timer, duration: u64) {
    // toggle timer for selected medicine to true
    timer.lock().unwrap().toggle(&med);

    // create duration struct
    let duration = Duration::from_secs(duration);
    sleep(duration).await;

    // toggle timer for selected medicine to false
    timer.lock().unwrap().toggle(&med);
    println!(
        "{}",
        format!("{} Can be taken again! Timer Done!", med).green()
    );
}

// connect to mongodb
pub async fn connect() -> Result<Database, mongodb::error::Error> {
    let uri = "mongodb://localhost:27017/";
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("prescriptions");
    Ok(db)
}

// get all actions from mongodb
pub async fn get_all_actions(
    actions: &Collection<Action>,
) -> Result<Vec<Action>, mongodb::error::Error> {
    let start_date = Local::now()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
    let end_date = Local::now()
        .with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap();

    let filter = doc! {
        "taken_at": {
            "$gte": start_date.to_rfc3339(),
            "$lt": end_date.to_rfc3339(),
        }
    };
    let options = FindOptions::default();
    let mut cursor = actions.find(filter, options).await?;
    let mut result: Vec<Action> = vec![];
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(act) => result.push(act),
            Err(e) => println!("Encountered error in collecting actions {}", e),
        }
    }
    Ok(result)
}

// get user choice
pub fn get_user_choice() -> Result<Medicine, std::io::Error> {
    println!("{}", "\nList of Medicines:".yellow());
    println!("{}", "1. Cephalexin".yellow());
    println!("{}", "2. Ibuprofen".yellow());
    println!("{}", "3. Oxycodone".yellow());
    println!("{}", "4. Lorazepam".yellow());
    println!("{}", "5. Allegra".yellow());

    let mut user_input = String::new();
    println!("{}", "Choose Medicine that is taken: ".red());

    let _ = std::io::stdin().read_line(&mut user_input);

    let choice: usize = user_input.trim().parse().expect("Invalid input");

    let selected_medicine = match choice {
        1 => Medicine::Cephalexin,
        2 => Medicine::Ibuprofen,
        3 => Medicine::Oxycodone,
        4 => Medicine::Lorazepam,
        5 => Medicine::Allegra,
        _ => {
            return get_user_choice(); // You can return a default medicine or handle this case as needed.
        }
    };

    Ok(selected_medicine)
}
