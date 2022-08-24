use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Debt {
    pub amount: f64,
    pub description: String,
    pub from: String,
    pub to: String,
}

pub fn create_uniform_expense(
    payer: String, 
    amount: f64,
    description: String,
    beneficiaries: Vec<String>
) -> Vec<Debt> {
    let mut debts = Vec::new();
    let debt_amount = amount / beneficiaries.len() as f64;
    for beneficiary in beneficiaries {
        if beneficiary == payer {
            continue;
        }
        debts.push(Debt {
            amount: debt_amount,
            description: description.clone(),
            from: beneficiary,
            to: payer.clone()
        });
    }
    debts
}

pub fn create_detailed_expense(
    payer: String,
    description: String,
    costs_per_person: Vec<(String, f64)>
) -> Vec<Debt> {
    let mut debts = Vec::new();
    for (beneficiary, cost) in costs_per_person {
        if beneficiary == payer {
            continue;
        }
        debts.push(Debt {
            amount: cost,
            description: description.clone(),
            from: beneficiary,
            to: payer.clone()
        });
    }
    debts
}

pub type Balance = Vec<(String, f64)>;

pub fn calculate_balance(debts: Vec<Debt>) -> Balance {
    let mut balance = HashMap::new();
    for debt in debts {
        let from_balance= balance.get(&debt.from).unwrap_or(&0.0);
        let to_balance = balance.get(&debt.to).unwrap_or(&0.0);
        let new_from_balance = *from_balance - debt.amount;
        let new_to_balance = *to_balance + debt.amount;
        balance.insert(debt.from, new_from_balance);
        balance.insert(debt.to, new_to_balance);
    }
    balance.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::{calculate_balance, create_detailed_expense, create_uniform_expense};
    use anyhow::Result;
    #[test]
    fn test_calculate_balance_uniform() -> Result<()> {
        let debts = create_uniform_expense(
            String::from("payer"),
            100.0,
            String::from("description"),
            vec![
                String::from("ben1"),
                String::from("ben2"),
                String::from("ben3"),
                String::from("payer"),
                ]
        );
        let balance = calculate_balance(debts).sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(balance, vec![
            (String::from("ben1"), -25.00), 
            (String::from("ben2"), -25.00), 
            (String::from("ben3"), -25.00),
            (String::from("payer"), 75.00) 
        ].sort_by(|a, b| a.0.cmp(&b.0))); 
        Ok(())
    }

    #[test]
    fn test_calculate_balance_detailed() -> Result<()> {
        let debts = create_detailed_expense(
            String::from("payer"),
            String::from("description"),
            vec![
                (String::from("ben1"), 5.00),
                (String::from("ben2"), 7.00),
                (String::from("ben3"), 9.00),
                (String::from("payer"), 2.00)
            ]
        );
        let balance = calculate_balance(debts).sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(balance, vec![
            (String::from("ben1"), -5.00), 
            (String::from("ben2"), -7.00), 
            (String::from("ben3"), -9.00),
            (String::from("payer"), 22.00) 
        ].sort_by(|a, b| a.0.cmp(&b.0))); 
        Ok(())
    }
}