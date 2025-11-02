use std::io;

#[derive(Debug)]
struct BankAccount{
    owner:String,
    balance:f64,
}

impl BankAccount {
    fn new(owner: String, initial_balance: f64) -> Self {
        Self {
            owner,
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64){
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited ${:.2}",amount)
        } else {
            println!("Deposit amount must be positive!");
        }
    }

    fn withdraw(&mut self, amount: f64){
        if amount <= 0.0 {
            println!("Withdrawal amount must be positive!");
        } else if amount > self.balance {
            println!("Insufficient funds! Current balance: ${:.2}", self.balance);
        } else {
            self.balance -= amount;
            println!("Withdrew ${:.2}", amount);
        }
    }

    fn check_balance(&self){
        println!("Current balance: ${:.2}", self.balance);
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter account owner name: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let owner = input.trim().to_string();

    let mut account = BankAccount::new(owner, 0.0);

    loop{
        println!("\n--- Bank Menu ---");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Exit");
        println!("Choose an option: ");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim();

        match choice {
            "1" => {
                input.clear();
                println!("Enter amount to deposit: ");
                io::stdin().read_line(&mut input).unwrap();
                let amount: f64 = input.trim().parse().unwrap_or(0.0);
                account.deposit(amount);
            }
            "2" => {
                input.clear();
                println!("Enter amount to withdraw: ");
                io::stdin().read_line(&mut input).unwrap();
                let amount: f64 = input.trim().parse().unwrap_or(0.0);
                account.withdraw(amount);
            }
            "3" => account.check_balance(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
    
}
