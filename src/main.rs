mod collections;
mod helper;
mod medicine;

use medicine::{MedicineTimer};
use std::sync::{Arc, Mutex};
use collections::{Action, Config};
use helper::{connect, get_all_actions, get_user_choice, async_sleep};
use colored::*;


#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    // config db
    let db = connect().await?;
    let actions = db.collection::<Action>("actions");
    let vec_actions: Vec<Action> = get_all_actions(&actions).await?;
    let mut current_config: Config = Config::from_actions(&vec_actions);
    let remaining_timer = current_config.create_timer_durations(&vec_actions);

    // Timer initialize
    let timer = Arc::new(Mutex::new(MedicineTimer::new()));

    // create new timers where the program had left off
    for (medicine, duration) in remaining_timer {
        if duration > 0 {
            // scoped lock to ensure it drops it
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
            println!("Remaining Time: {}", current_config.calculate_all_remaining(t).cyan());
        }
        let selected_medicine = get_user_choice()?;
        match current_config.check_and_insert(&selected_medicine) {
            true => {
                 let check = { 
                    let t = timer.lock().unwrap();
                    t.check(&selected_medicine)
                };

                if !check {
                    let act = Action::new(selected_medicine);
                    let _ = actions.insert_one(&act, None).await;
                    let t = timer.clone();

                    println!("Valid, dose... proceed");
                    println!("Starting Timer...");

                    let duration = current_config.get_default_duration(&selected_medicine);
                    tokio::spawn(async move {
                        async_sleep(selected_medicine, t, duration).await;
                    });
                } else {
                    println!("Timer in progress...");
                }
                
                
            },
            false => {
                println!("Exceeding dosage for the day...");
            }
        }
        
    };
}
