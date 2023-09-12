mod collections;
mod helper;
mod medicine;

use medicine::{MedicineTimer};
use std::sync::{Arc, Mutex};
use collections::{Action, Config};
use helper::{connect, get_all_actions, get_user_choice, async_sleep};



#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    // config db
    let db = connect().await?;
    let actions = db.collection::<Action>("actions");
    let vec_actions: Vec<Action> = get_all_actions(&actions).await?;
    let mut current_config: Config = Config::from_actions(vec_actions);

    // Timer initialize
    let timer = Arc::new(Mutex::new(MedicineTimer::new()));

    loop {
        // get user input
        let selected_medicine = get_user_choice(timer.clone())?;
        
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
                    tokio::spawn(async move {
                        async_sleep(selected_medicine, t).await;
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
