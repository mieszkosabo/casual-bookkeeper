use casual_bookkepper::{api::*, display::print_balance, expense::{calculate_balance, create_detailed_expense}};

use anyhow::Result;

fn main() -> Result<()> {
    erase_state()?;
    print_balance(&calculate_balance(create_detailed_expense(
        "Alice".into(),
        "Lunch".into(),
        vec![
            ("Bob".into(), 17.00), 
            ("Carol".into(), 22.40),
            ("Danny".into(), 13.40)
            ]
    )));
    Ok(())
}
