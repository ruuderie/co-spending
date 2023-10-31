use std::collections::{BTreeMap, HashMap};


fn main() {
    // We define a list of all people who are contributors to the expenses.
    let contributors = vec!["Colin", "Justin", "Arthur", "Quincy", "Zach", "Phil", "Terry", "Ruud", "Devontae"];

    // This is our main dataset. Each tuple represents an expense: who paid (payer), how much they paid (amount), and any exclusions
    // (people who don't need to contribute to this particular expense).
    let data = vec![
        // Here, Zach paid $259.95, and there are no exclusions, meaning everyone should contribute.
        ("Zach", 259.95, vec![]),
        ("Colin", 75.00, vec![]),
        ("Terry", 75.00, vec![]),
        ("Zach", 75.00, vec![]),
        ("Phil", 75.00, vec![]),
        ("Quincy", 121.73, vec![]),
        ("Quincy", 106.92, vec![]),
        ("Ruud", 110.00, vec![]),
        ("Ruud", 66.65, vec![]),
        ("Ruud", 163.17, vec![]),
        ("Ruud", 62.82, vec![]),
        ("Ruud", 66.55, vec![]),
        ("Phil", 375.00, vec![]),
        // Here, Justin paid $806, but Quincy and Ruud are excluded from contributing to this expense.
        ("Justin", 806.00, vec!["Quincy", "Ruud"]),
    ];

    // We're creating a 'ledger' of balances, which tracks how much each person owes to every other person.
    let mut detailed_balances: HashMap<&str, HashMap<&str, f64>> = HashMap::new();

    // For every expense in our dataset:
    for (_, &(payer, amount, ref exclusions)) in data.iter().enumerate() {
        // Calculate how many contributors should contribute to this particular expense.
        let num_contributors = contributors.len() - exclusions.len();
        // Calculate how much each contributor owes for this expense.
        let each_contributor_owes = amount / num_contributors as f64;

        // For every person in our list of contributors:
        for &contributor in &contributors {
            // If this person is not the one who paid and they are not excluded:
            if contributor != payer && !exclusions.contains(&contributor) {
                // Record in the ledger that this contributor owes the payer a specific amount.
                let entry = detailed_balances.entry(contributor).or_insert(HashMap::new());
                *entry.entry(payer).or_insert(0.0) += each_contributor_owes;
            }
        }
    }

    // Now, we'll simplify the ledger. If A owes B some amount and B owes A some amount, we'll net them off.
    let mut simplified_balances: BTreeMap<(&str, &str), f64> = BTreeMap::new();

    // For every balance in our detailed ledger:
    for (person, owes) in &detailed_balances {
        for (receiver, amount) in owes.iter() {
            // Check if the opposite person owes anything.
            if let Some(opposite_amount) = detailed_balances.get(*receiver).and_then(|m| m.get(person)) {
                // Calculate the net amount after offsetting what they owe each other.
                let net_amount = amount - opposite_amount;

                // If there's a positive net amount, record it.
                if net_amount > 0.0 {
                    simplified_balances.insert((person, receiver), net_amount);
                } else if net_amount < 0.0 {
                    // If it's negative, it means the opposite person should be the payer. Record it accordingly.
                    simplified_balances.insert((receiver, person), -net_amount);
                }
            } else if *amount > 0.0 {
                // If there's no opposite balance, just record the amount as is.
                simplified_balances.insert((person, receiver), *amount);
            }
        }
    }

    // Now, for every simplified balance:
    for ((debtor, creditor), _) in &simplified_balances {
        // We'll zero out the corresponding amount in the detailed ledger, so that we don't double count.
        if let Some(debtor_balances) = detailed_balances.get_mut(*debtor) {
            if let Some(balance) = debtor_balances.get_mut(*creditor) {
                *balance = 0.0;
            }
        }
    }

    // Finally, print out the simplified balances to see who owes whom.
    println!("Simplified Balances:");
    for ((debtor, creditor), amount) in &simplified_balances {
        println!("{} owes {}: ${:.2}", debtor, creditor, amount);
    }
}
