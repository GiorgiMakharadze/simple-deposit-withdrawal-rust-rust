use std::collections::HashMap;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i64,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn summary(&self) -> String {
        format!("{}", self)
    }

    fn deposit(&mut self, amount: i64) -> Result<i64, AccountError> {
        if amount < 0 {
            return Err(AccountError::NegativeAmount);
        }
        self.balance = self.balance.checked_add(amount).ok_or(AccountError::AmountOverflow)?;
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount: i64) -> Result<i64, AccountError> {
        if amount < 0 {
            return Err(AccountError::NegativeAmount);
        }
        if self.balance < amount {
            return Err(AccountError::InsufficientFunds);
        }
        self.balance -= amount;
        Ok(self.balance)
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let balance_dollars = self.balance as f64 / 100.0;
        write!(
            f,
            "Account {} ({}) has a balance of ${:.2}",
            self.id, self.holder, balance_dollars
        )
    }
}

#[derive(Debug)]
enum AccountError {
    NegativeAmount,
    InsufficientFunds,
    AmountOverflow,
    AccountNotFound,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::NegativeAmount => write!(f, "Amount cannot be negative"),
            AccountError::InsufficientFunds => write!(f, "Insufficient funds"),
            AccountError::AmountOverflow => write!(f, "Amount overflow"),
            AccountError::AccountNotFound => write!(f, "Account not found"),
        }
    }
}

impl Error for AccountError {}

#[derive(Debug)]
struct Bank {
    accounts: HashMap<u32, Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: HashMap::new(),
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.id, account);
    }

    fn total_balance(&self) -> i64 {
        self.accounts.values().map(|account| account.balance).sum()
    }

    fn summary(&self) -> String {
        self.accounts
            .values()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_account_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.get_mut(&id)
    }

    fn get_account(&self, id: u32) -> Option<&Account> {
        self.accounts.get(&id)
    }

    fn transfer(&mut self, from_id: u32, to_id: u32, amount: i64) -> Result<(), AccountError> {
        if amount < 0 {
            return Err(AccountError::NegativeAmount);
        }

        if !self.accounts.contains_key(&from_id) {
            return Err(AccountError::AccountNotFound);
        }
        if !self.accounts.contains_key(&to_id) {
            return Err(AccountError::AccountNotFound);
        }

        if from_id == to_id {
            return Ok(());
        }

        {
            let from_account = self.accounts.get(&from_id).unwrap();
            if from_account.balance < amount {
                return Err(AccountError::InsufficientFunds);
            }
        }

        {
            let from_account = self.accounts.get_mut(&from_id).unwrap();
            from_account.withdraw(amount)?;
        }

        {
            let to_account = self.accounts.get_mut(&to_id).unwrap();
            to_account.deposit(amount)?;
        }

        Ok(())
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_balance_dollars = self.total_balance() as f64 / 100.0;
        write!(f, "Bank total balance: ${:.2}", total_balance_dollars)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut bank = Bank::new();

    let mut account1 = Account::new(1, String::from("Giorgi"));
    let mut account2 = Account::new(2, String::from("QioJI"));

    account1.deposit(50000)?; 
    account1.withdraw(25000)?; 

    account2.deposit(30000)?; 

    bank.add_account(account1);
    bank.add_account(account2);

    bank.transfer(1, 2, 10000)?; 

    println!("{}", bank.summary());
    println!("{}", bank);

    Ok(())
}
