use crate::collections::Action;
use crate::medicine::{Medicine, Timer};

use futures::stream::StreamExt;
use chrono::Timelike;
use chrono::{Local};
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection, Database};
use mongodb::bson::doc;
use tokio::time::{sleep, Duration};

pub async fn async_sleep(med: Medicine, timer: Timer) {
    timer.lock().unwrap().toggle(&med);
        
    let duration = match med {
        Medicine::Cephalexin => Duration::from_secs(30),
        Medicine::Oxycodone => Duration::from_secs(30),
        Medicine::Ibuprofen => Duration::from_secs(30),
        Medicine::Lorazepam => Duration::from_secs(30),
        Medicine::Allegra => Duration::from_secs(30),
    };

    sleep(duration).await;

    timer.lock().unwrap().toggle(&med);
    println!("{} Can be taken again! Timer Done!", med);
}

pub async fn connect() -> Result<Database, mongodb::error::Error> {
    let uri = "mongodb://localhost:27017/";
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("prescriptions");
    Ok(db)
}

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
            Err(e) => println!("Encountered error in collecting actions {}", e)
        }
    }
    Ok(result)
}

pub fn get_user_choice(timer: Timer) -> Result<Medicine, std::io::Error> {
    println!("\nList of Medicines:");
    println!("1. Cephalexin");
    println!("2. Ibuprofen");
    println!("3. Oxycodone");
    println!("4. Lorazepam");
    println!("5. Allegra");
    println!("6. Remaining Time");

    let mut user_input = String::new();
    println!("Choose Medicine that is taken: ");

    let _ = std::io::stdin().read_line(&mut user_input);

    let choice: usize = user_input.trim().parse().expect("Invalid input");

    let selected_medicine = match choice {
        1 => Medicine::Cephalexin,
        2 => Medicine::Oxycodone,
        3 => Medicine::Ibuprofen,
        4 => Medicine::Lorazepam,
        5 => Medicine::Allegra,
        _ => {
            println!("\n {}", timer.lock().unwrap().calculate_elapsed_time());
            return get_user_choice(timer); // You can return a default medicine or handle this case as needed.
        }
    };

    Ok(selected_medicine)
}
