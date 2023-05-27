use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::io;
use std::io::BufRead;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// Usage:
// cargo run < ./operations.jsonl

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
enum Input {
    #[serde(rename = "account")]
    Account(Account),
    #[serde(rename = "transaction")]
    Transaction(Transaction),
}

fn main() {
    for line in io::stdin().lock().lines() {
        if let Ok(data) = line {
            let deserialization_result: Result<Input, serde_json::Error> =
                serde_json::from_str::<Input>(&data);
            let res: Input = deserialization_result.expect("This should not panic");

            match res {
                Input::Account(res) => println!(
                    "<Account>: active_card: {:?}, available_limit: {:?}",
                    res.active_card, res.available_limit
                ),
                Input::Transaction(res) => println!(
                    "<Transaction>: merchant: {:?}, amount: {:?}, time: {:?}",
                    res.merchant, res.amount, res.time
                ),
            }
        }
    }
}
