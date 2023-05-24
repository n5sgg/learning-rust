use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "active-card")]
    active_card: bool,
    #[serde(rename = "available-limit")]
    available_limit: u32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
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
enum State {
    Inactive,
    Active,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AccountLog {
    log: Vec<Account>,
    state: State,
}

impl AccountLog {
    pub fn new() -> Self {
        AccountLog { state: State::Inactive, log: Vec::new() }
    }

    pub fn process(&mut self, record: Record) {
        match (&self.state, record) {
            (State::Inactive, Record::Account(acc @ Account { active_card: true, .. })) => {
                self.log.push(acc);
                self.state = State::Active;
            }
            (State::Active, Record::Transaction(tx)) => {
                let curr: &Account = self.log.last().unwrap(); // this is safe because the state is only active after pushing the first record to the log.
                if curr.available_limit >= tx.amount {
                    self.log.push(Account{active_card: curr.active_card, available_limit: curr.available_limit-tx.amount})
                }
            },
            (State::Inactive, Record::Account(acc @ Account{ active_card: false, .. })) => self.log.push(acc),
            (State::Inactive, Record::Transaction(_)) => (),
            (State::Active, Record::Account(_)) => (),
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
    use time_macros::datetime;

    #[test]
    fn test_accountlog_process() {
        let mut acclog = AccountLog::new();
        acclog.process(Record::Account(Account {
            active_card: true,
            available_limit: 100,
        }));
        acclog.process(Record::Transaction(Transaction {
            merchant: "Burger King".to_string(),
            amount: 20,
            time: datetime!(2019-02-13 11:00:00.00 +00:00),
        }));
        let exp = vec![
            Account {
                active_card: true,
                available_limit: 100,
            },
            Account {
                active_card: true,
                available_limit: 80,
            },
        ];
        assert_eq!(acclog.log, exp);
    }
}
