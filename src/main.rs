use std::result::Result;

// Define the Account trait with deposit, withdraw, and balance methods
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Define a struct BankAccount that implements the Account trait
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: u32, holder_name: &str) -> Self {
        BankAccount {
            account_number,
            holder_name: holder_name.to_string(),
            balance: 0.0,
        }
    }
}

impl Account for BankAccount {
    // Modify deposit method to return a Result<(), String>
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount >= 0.0 {
            self.balance += amount;
            Ok(())
        } else {
            Err("Deposit amount must be non-negative".to_string())
        }
    }

    // Modify withdraw method to return a Result<(), String>
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount >= 0.0 && amount <= self.balance {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Withdrawal amount must be non-negative and not exceed the balance".to_string())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount::new(1, "John Doe");
    let mut account2 = BankAccount::new(2, "Alice Smith");

    // Deposit money into account1 and handle errors
    match account1.deposit(100.0) {
        Ok(_) => println!("Deposit to account 1 successful"),
        Err(error) => println!("Error: {}", error),
    }

    // Withdraw money from account2 and handle errors
    match account2.withdraw(50.0) {
        Ok(_) => println!("Withdraw from account 2 successful"),
        Err(error) => println!("Error: {}", error),
    }

    // Print the balances of both accounts
    println!("Account 1 balance: ${}", account1.balance());
    println!("Account 2 balance: ${}", account2.balance());
}
