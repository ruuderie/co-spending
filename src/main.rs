use std::collections::HashMap;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

// Define an enum to represent different expense categories.
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum FoodExpense {
    Groceries,
    Chef,
    Restaurant,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum ExpenseCategory {
    Transport,
    TransportDamange,
    BottleService,
    Liquor,
    Other,
    Backwoods,
    Food(FoodExpense),
    Restaurant,
    Airbnb
}
// Define the Expense struct which holds information about each expense.
#[derive(Debug)]
struct Expense<'a> {
    payer: &'a str,
    amount: f64,
    exclusions: Vec<&'a str>,
    category: ExpenseCategory,
    received_funds: Option<f64>,
    to_be_paid_by: Option<&'a str>,
}

fn main() {
    let contributors = vec!["Colin", "Justin", "Arthur", "Quincy", "Zach", "Phil", "Terry", "Ruud", "Devontae"];

    // Initialize a prepaid amount for food-related expenses.
    let mut prepaid_amounts: HashMap<ExpenseCategory, f64> = HashMap::new();
    // Assume we have a prepaid amount for food and transport, managed by Quincy
    prepaid_amounts.insert(ExpenseCategory::Food(FoodExpense::Groceries), 820.00);
    prepaid_amounts.insert(ExpenseCategory::Transport, 720.00);



    let data = vec![
        Expense { payer: "Colin", amount: 75.00, exclusions: vec![], category: ExpenseCategory::Backwoods, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Terry", amount: 75.00, exclusions: vec![], category: ExpenseCategory::Backwoods, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Zach", amount: 75.00, exclusions: vec![], category: ExpenseCategory::Backwoods, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Phil", amount: 75.00, exclusions: vec![], category: ExpenseCategory::Backwoods, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Ruud", amount: 110.00, exclusions: vec![], category: ExpenseCategory::Liquor, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Ruud", amount: 66.65, exclusions: vec![], category: ExpenseCategory::Food(FoodExpense::Groceries), received_funds: None, to_be_paid_by: None },
        Expense { payer: "Ruud", amount: 163.17, exclusions: vec![], category: ExpenseCategory::Liquor, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Ruud", amount: 66.55, exclusions: vec![], category: ExpenseCategory::TransportDamange, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Phil", amount: 375.00, exclusions: vec![], category: ExpenseCategory::Food(FoodExpense::Groceries), received_funds: None, to_be_paid_by: None },
        Expense { payer: "Justin", amount: 806.00, exclusions: vec!["Quincy", "Ruud"], category: ExpenseCategory::Restaurant, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Zach", amount: 163.00, exclusions: vec![], category: ExpenseCategory::BottleService, received_funds: None, to_be_paid_by: None },
        Expense { payer: "Quincy", amount: 186.00, exclusions: vec!["Ruud"], category: ExpenseCategory::Food(FoodExpense::Restaurant), received_funds: None, to_be_paid_by: None },
        Expense { payer: "Quincy", amount: 47.00, exclusions: vec![], category: ExpenseCategory::Airbnb, received_funds: None, to_be_paid_by: None },
    ];
    // Initialize HashMaps for keeping track of debts and net balances.
    let mut debts: HashMap<(&str, &str), Vec<f64>> = HashMap::new();
    let mut net_balances: HashMap<&str, f64> = HashMap::new();

    // Process each expense
    for expense in &data {
        let received_funds = expense.received_funds.unwrap_or(0.0);
        let mut amount_to_cover = expense.amount - received_funds;

        if let Some(prepaid_amount) = expense.to_be_paid_by.filter(|&p| p == "Quincy").and_then(|_| prepaid_amounts.get_mut(&expense.category)) {
            if expense.payer != "Quincy" {
                *prepaid_amount -= amount_to_cover;
                debts.entry((expense.payer, "Quincy")).or_insert_with(|| vec![]).push(amount_to_cover);
            }
            continue;
        }

        let split_between = contributors.iter().filter(|&c| !expense.exclusions.contains(c) && *c != expense.payer).count();
        if split_between > 0 {
            let each_contributor_owes = amount_to_cover / split_between as f64;
            for &contributor in &contributors {
                if !expense.exclusions.contains(&contributor) && contributor != expense.payer {
                    // Check if a debt between these two people already exists
                    if let Some(debt) = debts.get_mut(&(contributor, expense.payer)) {
                        // Add to the existing debt
                        debt.push(each_contributor_owes);
                    } else {
                        // Create a new debt entry
                        debts.insert((contributor, expense.payer), vec![each_contributor_owes]);
                    }
                }
            }
        }
    }

    // Summarize debts into a net amount owed between each pair of individuals
    let mut net_debts: HashMap<(&str, &str), f64> = HashMap::new();
    for (&(debtor, creditor), amounts) in debts.iter() {

        let total: f64 = amounts.iter().sum();
        *net_debts.entry((debtor, creditor)).or_insert(0.0) += total;
        *net_balances.entry(debtor).or_insert(0.0) -= total;
        *net_balances.entry(creditor).or_insert(0.0) += total;
    }

    // Now, for each pair of individuals, print the detailed transactions and the net amount owed
    for ((debtor, creditor), transactions) in debts.iter() {
        println!("{} owes {}:", debtor, creditor);
        for amount in transactions {
            println!("    ${:.2}", amount);
        }
        let total_debt: f64 = transactions.iter().sum();
        println!("{} owes {} a total of: ${:.2}\n", debtor, creditor, total_debt);
    }

    // Print out the net amount each person should pay or receive
    for (person, balance) in net_balances.iter() {
        if *balance < 0.0 {
            println!("{} should pay a total of: ${:.2}", person, -*balance);
        } else if *balance > 0.0 {
            println!("{} should receive a total of: ${:.2}", person, *balance);
        }
    }
    // Before printing the table, sort the debt entries by debtor.
    let mut sorted_debtors: Vec<(&str, &str)> = debts.keys().cloned().collect();
    sorted_debtors.sort_by(|a, b| a.0.cmp(b.0));

    // Initialize a pretty table.
    let mut table = Table::new();
    table.add_row(row!["Debtor", "Creditor", "Individual Transactions", "Total Debt"]);

    // Now, for each pair of individuals in the sorted list, add the transactions and the net amount owed to the table
    for &(debtor, creditor) in &sorted_debtors {
        if let Some(transactions) = debts.get(&(debtor, creditor)) {
            let transaction_details = transactions.iter()
                .map(|amount| format!("${:.2}", amount))
                .collect::<Vec<_>>()
                .join("\n");

            let total_debt: f64 = transactions.iter().sum();
            table.add_row(Row::new(vec![
                Cell::new(debtor),
                Cell::new(creditor),
                Cell::new(&transaction_details),
                Cell::new(&format!("${:.2}", total_debt)),
            ]));
        }
    }

    // Print the table to the console
    table.printstd();


}

