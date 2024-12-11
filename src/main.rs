#[derive(Debug, Clone)]
struct Account {
    id: u32,
    balance: isize,
    holder: String,
}

impl Account {
    fn new(holder: String) -> Self {
        Account {
            id: 0,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, some_money: isize) -> isize {
        self.balance += some_money;
        self.balance
    }

    fn withdraw(&mut self, some_money: isize) -> isize {
        self.balance -= some_money;
        self.balance
    }

    fn get_summary(&self) -> String {
        let summary = format!(
            "Account ID: {}, Holder: {}, Balance: {}",
            self.id, self.holder, self.balance
        );
        summary
    }
}

#[derive(Debug, Clone)]
struct Bank {
    last_id: u32,
    name: String,
    accounts: Vec<Account>,
}

impl Bank {
    fn new(name: String) -> Self {
        Bank {
            last_id: 0,
            name,
            accounts: vec![],
        }
    }

    fn add_account(&mut self, account: &mut Account) -> () {
        self.last_id += 1;
        account.id = self.last_id;
        self.accounts.push(account.clone());
    }

    fn calculate_total_balance(&self) -> isize {
        self.accounts.iter().map(|acc| acc.balance).sum()
        // ^-- makes the same as code below --v
        // let mut total: isize = 0;
        // if self.accounts.len() > 0 {
        //     for acc in &self.accounts {
        //         total = total + acc.balance;
        //     }
        // }
        // total
    }

    fn get_summaries(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|acc| acc.get_summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new("International Bank".to_string());
    let mut account_1 = Account::new("John Doe".to_string());
    let mut account_2 = Account::new("Ian McDoe".to_string());
    let mut account_3 = Account::new("Lisa Murrow".to_string());
    // let account = Account::new(1, String::from("John Doe"));

    account_1.deposit(3300);
    account_1.deposit(120);
    account_1.deposit(200);
    account_1.withdraw(3015);
    account_1.withdraw(125);

    account_2.deposit(302);
    account_2.deposit(584);
    account_2.deposit(20);
    account_2.withdraw(53);
    account_2.withdraw(65);

    account_3.deposit(9494);
    account_3.deposit(384);
    account_3.deposit(65);
    account_3.withdraw(744);
    account_3.withdraw(1250);

    println!("Account summary: {:#?}", account_1.get_summary());

    bank.add_account(&mut account_1);
    bank.add_account(&mut account_2);
    bank.add_account(&mut account_3);

    println!("Bank: {:#?}", bank);
    println!("Bank total balance: {:#?}", bank.calculate_total_balance());
    println!("Bank summaries: {:#?}", bank.get_summaries());
}
