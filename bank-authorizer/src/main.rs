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
pub struct AccountLog(Vec<Account>);

impl AccountLog {
    pub fn new() -> Self {
        AccountLog(Vec::new())
    }

    pub fn process(&mut self, data: Record) {
        if let Some(acc) = self.0.last() {
            match data {
                Record::Transaction(tx) => {
                    if acc.available_limit >= tx.amount {
                        self.0.push(Account {
                            active_card: acc.active_card,
                            available_limit: acc.available_limit - tx.amount,
                        })
                    }
                }
                Record::Account(_) => (),
            }
        } else {
            match data {
                Record::Account(acc) => self.0.push(acc),
                Record::Transaction(_) => (),
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
    use time_macros::datetime;

    #[test]
    fn test_accountlog_process() {
        let mut acclog = AccountLog::new();
        acclog.process(Record::Account(Account {
            active_card: false,
            available_limit: 100,
        }));
        acclog.process(Record::Transaction(Transaction {
            merchant: "Burger King".to_string(),
            amount: 20,
            time: datetime!(2019-02-13 11:00:00.00 +00:00),
        }));
        let exp = vec![
            Account {
                active_card: false,
                available_limit: 100,
            },
            Account {
                active_card: false,
                available_limit: 80,
            },
        ];
        assert_eq!(acclog.0, exp);
    }
}
