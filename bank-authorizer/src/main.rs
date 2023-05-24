use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use time_macros::datetime;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "active-card")]
    active_card: bool,
    #[serde(rename = "available-limit")]
    available_limit: u32,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    merchant: String,
    amount: u32,
    #[serde_as(as = "Rfc3339")]
    time: OffsetDateTime,
}

#[derive(Debug)]
pub enum Record {
    Account(Account),
    Transaction(Transaction),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DoubleEntry {
    account: Account,
    tx: Transaction,
}

impl DoubleEntry {
    fn new(acc: Account) -> Self {
        DoubleEntry {
            account: acc,
            tx: Transaction {
                merchant: "".to_string(),
                amount: 0,
                time: datetime!(0001-01-01 00:00:00.00 +00:00),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Inactive,
    Active,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AccountLog {
    log: Vec<DoubleEntry>,
    state: State,
}

#[derive(Debug)]
pub enum AccountError {
    NotInitialized,
    InsufficientLimit,
    AlreadyInitialized,
    CardNotActive,
    DoubleTransaction,
    // HighFrequencySmallInterval,
}

impl std::error::Error for AccountError {}
impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountError::NotInitialized => write!(f, "account-not-initialized"),
            AccountError::InsufficientLimit => write!(f, "insufficient-limit"),
            AccountError::AlreadyInitialized => write!(f, "account-not-initialized"),
            AccountError::CardNotActive => write!(f, "card-not-active"),
            AccountError::DoubleTransaction => write!(f, "doubled-transaction"),
            // AccountError::HighFrequencySmallInterval => write!(f, "high-frequency-small-interval"),
        }
    }
}

impl AccountLog {
    pub fn new() -> Self {
        AccountLog {
            state: State::Inactive,
            log: Vec::new(),
        }
    }

    pub fn process(&mut self, record: Record) -> Result<Account, AccountError> {
        match (&self.state, record) {
            (State::Inactive, Record::Account(acc)) => {
                if let Some(_) = self.log.first() {
                    return Err(AccountError::AlreadyInitialized);
                }
                self.log.push(DoubleEntry::new(acc));
                if acc.active_card {
                    self.state = State::Active;
                }
                Ok(acc)
            }
            (State::Inactive, Record::Transaction(_)) => match self.log.first() {
                None => Err(AccountError::NotInitialized),
                Some(_) => Err(AccountError::CardNotActive),
            },
            (State::Active, Record::Account(_)) => Err(AccountError::AlreadyInitialized),
            (State::Active, Record::Transaction(tx)) => {
                let last_entry: &DoubleEntry = self.log.last().unwrap(); // this is safe because the state is only active after pushing the first record to the log.

                if last_entry.tx == tx {
                    return Err(AccountError::DoubleTransaction);
                }

                if last_entry.account.available_limit < tx.amount {
                    return Err(AccountError::InsufficientLimit);
                }

                let new_account_entry = Account {
                    available_limit: last_entry.account.available_limit - tx.amount,
                    active_card: last_entry.account.active_card,
                };
                let entry = DoubleEntry {
                    account: new_account_entry,
                    tx: tx,
                };
                self.log.push(entry);
                Ok(new_account_entry)
            }
        }
    }
}

fn main() {
    let acc = Account {
        active_card: true,
        available_limit: 100,
    };
    println!(
        "active_card: {:?}, available_limit: {:?}",
        acc.active_card, acc.available_limit
    );
    let data: &str = r#"
        {
            "merchant": "Burguer King",
            "amount": 20,
            "time": "2022-07-08T09:10:11Z"
        }
    "#;
    let deserialization_result: Result<Transaction, serde_json::Error> =
        serde_json::from_str::<Transaction>(data);
    let res: Transaction = deserialization_result.expect("This should not panic");
    println!(
        "merchant: {:?}, amount: {:?}, time: {:?}",
        res.merchant, res.amount, res.time
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accountlog_process() {
        let mut acclog = AccountLog::new();
        assert_eq!(
            acclog
                .process(Record::Account(Account {
                    active_card: true,
                    available_limit: 100,
                }))
                .expect("should not panic"),
            Account {
                active_card: true,
                available_limit: 100,
            }
        );

        assert_eq!(
            acclog
                .process(Record::Transaction(Transaction {
                    merchant: "Burger King".to_string(),
                    amount: 20,
                    time: datetime!(2019-02-13 11:00:00.00 +00:00),
                }))
                .expect("should not panic"),
            Account {
                active_card: true,
                available_limit: 80,
            }
        );
    }
}
