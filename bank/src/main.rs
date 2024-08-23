// Ownership and borrowing in a nutshell:
// Multiple things can refer to a value at the same time, but they are all read-only
// A value can only be updated when there are no read-only references to it
// This forms the basis of ownership and borrowing in Rust
// The goal is to reduce the side effects of heap updates

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account)
}

fn main() {
    // let bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    print_account(account);
    print_account(account);
}
