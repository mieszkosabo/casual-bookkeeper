use crate::expense::Balance;

use colored::Colorize;

const MAX_DEBT_STRIP_LEN: i32 = 50;

fn create_debt_strip(max_value: &f64, person_balance: &f64) -> String {
    let debt_strip_len = (person_balance.abs() / max_value * MAX_DEBT_STRIP_LEN as f64) as usize;
    let personal_balance_len = person_balance.to_string().len();
    let debt_strip = person_balance
        .to_string() + &" ".repeat(debt_strip_len - personal_balance_len);

    if person_balance < &0.0 {
        debt_strip.on_red().bold().bright_white().to_string()
    } else {
        debt_strip.on_green().bold().bright_white().to_string()
    }
}

pub fn print_balance(balance: &Balance) -> () {
    println!("{}", "BALANCE SUMMARY".bold().white());
    balance
        .iter()
        .map(|(_, value)| value.abs())
        .max_by(|a, b | a.partial_cmp(b).unwrap())
        .map(|max_value| {
            let mut balance_copy = balance
                .clone();
            balance_copy
                .sort_by(|a, b| {
                    match a.1.abs().partial_cmp(&b.1.abs()).unwrap() {
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                    }
            });
            for (person, person_balance) in balance_copy {
                println!("{}:\t{}", person, create_debt_strip(&max_value, &person_balance));
            }
        });

    
}

