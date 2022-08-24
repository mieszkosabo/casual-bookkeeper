use crate::expense::Balance;

use colored::Colorize;

const MAX_DEBT_STRIP_LEN: i32 = 50;

fn create_debt_strip(max_value: &f64, person_balance: &f64) -> String {
    let debt_strip_len = 1.max((person_balance.abs() / max_value * MAX_DEBT_STRIP_LEN as f64) as usize) as usize;
    let personal_balance_len = person_balance.to_string().len();
    if debt_strip_len >= personal_balance_len {
        let debt_strip = person_balance
            .to_string() + &" ".repeat(debt_strip_len - personal_balance_len);
        if person_balance < &0.0 {
            return debt_strip.on_red().bold().bright_white().to_string();
        } else {
            return debt_strip.on_green().bold().bright_white().to_string();
        }
    } else {
        let balance_as_str = person_balance.to_string();
        let (strip_part, no_strip_part) = balance_as_str
            .split_at(debt_strip_len);
        if person_balance < &0.0 {
            return strip_part.on_red().bold().bright_white().to_string() + no_strip_part;
        } else {
            return strip_part.on_green().bold().bright_white().to_string() + no_strip_part;
        }
    }

}

pub fn print_balance(balance: &Balance) -> () {
    println!("{}\n", "BALANCE SUMMARY".bold().white());
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
            let longest_name_len = balance_copy
                .iter()
                .map(|(name, _)| name.len())
                .max()
                .unwrap(); // we know that the balance list in not empty
            for (person, person_balance) in balance_copy {
                println!("{:width$} {}", person, create_debt_strip(&max_value, &person_balance), width=longest_name_len);
            }
        });

    
}

