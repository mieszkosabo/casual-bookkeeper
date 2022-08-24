use crate::expense::Balance;

use anyhow::{Result, Context};

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

pub fn settle(balance: &Balance) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();
    let mut debtors = Vec::new();
    let mut creditors = Vec::new();

    for (name, amount) in balance.iter() {
        if amount < &0.0 {
            debtors.push((name.to_string(), amount.abs()));
        } else {
            creditors.push((name.to_string(), amount.clone()));
        }
    }

    while !debtors.is_empty() && !creditors.is_empty() {
        let (debtor_name, debt) = debtors.pop().context("Invalid debtors list")?;
        let (creditor_name, credit) = creditors.pop().context("Invalid creditor list")?;

        let sum_to_transfer = debt.min(credit);
        transactions.push(Transaction {
            from: debtor_name.clone(),
            to: creditor_name.clone(),
            amount: f64::trunc(sum_to_transfer * 100.0) / 100.0,
        });
       
        if debt > sum_to_transfer {
            debtors.push((debtor_name, debt - sum_to_transfer));

        } else {
            creditors.push((creditor_name, credit - sum_to_transfer));
        }
    }

    return Ok(transactions);
}

#[cfg(test)]
mod test {
    use super::{settle, Transaction};
    use anyhow::Result;
    use crate::expense::Balance;

    #[test]
    fn test_settle() -> Result<()> {
        let balance_test1: Balance = vec![
            ("Alice".into(), -42.01),
            ("Bob".into(), 168.15),
            ("Carol".into(), -13.99),
            ("Dave".into(), 29.01),
            ("Evan".into(), -286.77),
            ("Frank".into(), -132.48),
            ("Gil".into(), 351.97),
            ("Ines".into(), 3.34),
            ("Julia".into(), -63.23),
            ("Kurt".into(), -13.98),
        ].into_iter().collect();

        let mut transactions = settle(&balance_test1)?;

        transactions.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());

        // always n - 1 transactions should suffice
        assert_eq!(transactions.len(), balance_test1.len() - 1);
        assert_eq!(transactions[0], Transaction {
            from: "Kurt".into(),
            to: "Ines".into(),
            amount: 3.34
        });
        assert_eq!(transactions[1], Transaction {
            from: "Kurt".into(),
            to: "Gil".into(),
            amount: 10.64
        });
        assert_eq!(transactions[2], Transaction {
            from: "Carol".into(),
            to: "Bob".into(),
            amount: 13.99
        });
        assert_eq!(transactions[3], Transaction {
            from: "Evan".into(),
            to: "Dave".into(),
            amount: 29.01
        });
        assert_eq!(transactions[4], Transaction {
            from: "Alice".into(),
            to: "Bob".into(),
            amount: 42.01
        });
        assert_eq!(transactions[5], Transaction {
            from: "Julia".into(),
            to: "Gil".into(),
            amount: 63.23
        });
        assert_eq!(transactions[6], Transaction {
            from: "Evan".into(),
            to: "Bob".into(),
            amount: 112.13
        });
        assert_eq!(transactions[7], Transaction {
            from: "Frank".into(),
            to: "Gil".into(),
            amount: 132.47
        });
        assert_eq!(transactions[8], Transaction {
            from: "Evan".into(),
            to: "Gil".into(),
            amount: 145.62
        });

        Ok(())
    }

    #[test]
    fn works_for_empty_balance() -> Result<()> {
        let balance: Balance = vec![];
        let transactions = settle(&balance)?;
        assert_eq!(transactions.len(), 0);
        Ok(())
    }
}