mod collections;
mod helper;
mod medicine;

use collections::{Action, Config};
use helper::{connect, get_all_actions, get_user_choice};


#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    // config db
    let db = connect().await?;
    let actions = db.collection::<Action>("actions");
    let vec_actions: Vec<Action> = get_all_actions(&actions).await?;
    let mut current_config: Config = Config::from_actions(vec_actions);

    loop {
        // get user input
        let selected_medicine = get_user_choice()?;
        
        match current_config.check_and_insert(&selected_medicine) {
            true => {
                let act = Action::new(selected_medicine);
                let _ = actions.insert_one(&act, None).await;
                println!("Valid, dose... proceed");
            },
            false => {
                println!("Exceeding dosage for the day...")
            }
        }

        // let sleep_handle = async_sleep(selected_medicine);
        // sleep_handle.await;
        // Continue to the next iteration to take user input for the next medicine.
    }
}
