use casual_bookkepper::api::*;

use anyhow::Result;

fn main() -> Result<()> {
    erase_state()?;
    if let Ok(id) = create_group(String::from("test")) {
        println!("Created group with id: {}", &id);
        if add_member(&id, "Ziomeczek".into()).is_ok() {
            println!("Added member to group");
        }
        if add_member(&id, "Ziomeczek 2".into()).is_ok() {
            println!("Added member to group");
        }
        if add_member(&id, "Ziomeczek 2".into()).is_err() {
            println!("Couldn't add member of the same name to the group");
        }
        if remove_member(&id, "Ziomeczek".into()).is_ok() {
            println!("Removed member from group");
        }

        if remove_group(&id).is_ok() {
            println!("Removed group");
        }
    }

    Ok(())
}
