use time::OffsetDateTime;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug)]
struct BookAccount {
    account: String,
    amount: f64,
    off_balance: bool,
}

#[allow(dead_code)]
struct Ledger {
    currency: String,
    card_active: bool,
    accounts: Vec<BookAccount>,
    journal: Vec<Entry>,
}

impl Ledger {
    fn new() -> Ledger {
        Self {
            currency: "EUR".to_string(),
            card_active: true,
            accounts: vec![
                BookAccount {
                    account: "asset:settled".to_string(),
                    amount: 0.00,
                    off_balance: false,
                },
                BookAccount {
                    account: "liability:payable".to_string(),
                    amount: 0.00,
                    off_balance: false,
                },
                BookAccount {
                    account: "equity:interchange".to_string(),
                    amount: 0.00,
                    off_balance: false,
                },
                BookAccount {
                    account: "asset:unsettled".to_string(),
                    amount: 0.00,
                    off_balance: true,
                },
                BookAccount {
                    account: "liability:unsettled_cp".to_string(),
                    amount: 0.00,
                    off_balance: true,
                },
                BookAccount {
                    account: "asset:current_limit".to_string(),
                    amount: 0.00,
                    off_balance: true,
                },
                BookAccount {
                    account: "liability:current_limit_cp".to_string(),
                    amount: 0.00,
                    off_balance: true,
                },
                BookAccount {
                    account: "asset:max_limit".to_string(),
                    amount: 0.00,
                    off_balance: true,
                },
                BookAccount {
                    account: "liability:max_limit_cp".to_string(),
                    amount: 0.00,
                    off_balance: true,
                },
            ],
            journal: Vec::new(),
        }
    }

    fn append_entry(&mut self, entry: Entry) {
        self.journal.push(entry);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Entry {
    id: Uuid,
    amount: u32, // always positive
    debit_account: String,
    credit_account: String,
    post_date: OffsetDateTime, // the day the entry actually ocurred
    // movement: Movement,
}

fn main() {
    let mut ledger = Ledger::new();
    let now = OffsetDateTime::now_utc();

    ledger.append_entry(Entry{
        id: Uuid::new_v4(),
        amount: 15000,
        debit_account: "asset:max_limit".to_string(),
        credit_account: "liability:max_limit_cp".to_string(),
        post_date: now,
    });

    println!(
        "{:?}",
        ledger.journal
    );
}
