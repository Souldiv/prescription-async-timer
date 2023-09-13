mod collections;
mod helper;
mod medicine;

use collections::{Action, Config};
use colored::*;
use helper::{async_sleep, connect, get_all_actions, get_user_choice};
use medicine::MedicineTimer;
use std::sync::{Arc, Mutex};

use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    // config db
    let db = connect().await?;
    let actions = db.collection::<Action>("actions");
    let vec_actions: Vec<Action> = get_all_actions(&actions).await?;
    let mut current_config: Config = Config::from_actions(&vec_actions);
    let remaining_timer = current_config.create_timer_durations(&vec_actions);
    let md = current_config.create_timer_from_actions(&vec_actions);

    println!("{:?}", md);
    // Timer initialize
    let timer = Arc::new(Mutex::new(md));

    // create new timers where the program had left off
    for (medicine, duration) in remaining_timer {
        if duration > 0 {
            let t = timer.clone();
            tokio::spawn(async move {
                async_sleep(medicine, t, duration).await;
            });
        }
    }

    loop {
        // get user input
        {
            let t = timer.clone();
            let c = current_config.clone();
            let _ = spawn_blocking(move || {
                println!("Remaining Time {}", c.calculate_all_remaining(t).cyan());        
            }).await;
        }

        let selected_medicine = get_user_choice()?;

        let t = timer.lock().unwrap();
        match t.check(&selected_medicine) {
            false => {
                match current_config.check_and_insert(&selected_medicine) {
                        true => {
                            let act = Action::new(selected_medicine);
                            let _ = actions.insert_one(&act, None).await;
                            let t = timer.clone();
        
                            println!("Valid, dose... proceed");
                            println!("Starting Timer...");
        
                            let duration = current_config.get_default_duration(&selected_medicine);
                            tokio::spawn(async move {
                                async_sleep(selected_medicine, t, duration).await;
                            });
                        }
                        false => {
                            println!("Exceeding dosage for the day...");
                        }
                    }
                },
            true => {
                println!("Timer in progress...");
            }
        }
    }
        
}
