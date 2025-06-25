use std::io;

#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    InvalidAmount,
}

struct BankAccount {
    account_holder: String,
    balance: f64,
    account_number: String,
}

impl BankAccount {
    // Create a new bank account
    fn new(account_holder: String, initial_balance: f64, account_number: String) -> BankAccount {
        BankAccount {
            account_holder,
            balance: initial_balance,
            account_number,
        }
    }

    // Deposit money into the account
    fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount);
        }
        self.balance += amount;
        Ok(())
    }

    // Withdraw money from the account
    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount);
        }
        if self.balance < amount {
            return Err(BankError::InsufficientFunds);
        }
        self.balance -= amount;
        Ok(())
    }

    // Check the balance
    fn check_balance(&self) -> f64 {
        self.balance
    }
}

pub fn run() {
    let mut account = BankAccount::new("John Doe".to_string(), 1000.0, "123456789".to_string());

    loop {
        println!("Welcome to the Bank System");
        println!("1. View Balance");
        println!("2. Deposit Money");
        println!("3. Withdraw Money");
        println!("4. Exit");
        println!("Enter your choice (1-4):");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

        match choice {
            1 => {
                // View balance
                println!("Current balance: ${:.2}", account.check_balance());
            }
            2 => {
                // Deposit money
                println!("Enter the amount to deposit:");
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");
                let amount: f64 = amount.trim().parse().expect("Please enter a valid number");

                match account.deposit(amount) {
                    Ok(()) => println!(
                        "You have deposited ${:.2}. Your new balance is ${:.2}.",
                        amount,
                        account.check_balance()
                    ),
                    Err(BankError::InvalidAmount) => {
                        println!("Invalid amount! Please enter a positive number.")
                    }
                    _ => {}
                }
            }
            3 => {
                // Withdraw money
                println!("Enter the amount to withdraw:");
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");
                let amount: f64 = amount.trim().parse().expect("Please enter a valid number");

                match account.withdraw(amount) {
                    Ok(()) => println!(
                        "You have withdrawn ${:.2}. Your new balance is ${:.2}.",
                        amount,
                        account.check_balance()
                    ),
                    Err(BankError::InsufficientFunds) => println!(
                        "Insufficient funds! Your balance is ${:.2}.",
                        account.check_balance()
                    ),
                    Err(BankError::InvalidAmount) => {
                        println!("Invalid amount! Please enter a positive number.")
                    }
                    _ => {}
                }
            }
            4 => {
                // Exit the program
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }
    }
}
