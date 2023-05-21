use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Account {
    #[serde(rename = "active-card")]
    active_card: bool,
    #[serde(rename = "available-limit")]
    available_limit: u32,
}

impl Account {
    fn new(active: bool, limit: u32) -> Self {
        Self { active_card: active, available_limit: limit }
    }
}

#[derive(Debug)]
#[serde_as]
#[derive(Serialize, Deserialize)]
struct Transaction {
    merchant: String,
    amount: u32,
    #[serde_as(as = "Rfc3339")]
    time: OffsetDateTime,
}

impl Transaction {
    #[allow(dead_code)]
    fn new(merchant: String, amount: u32, time: OffsetDateTime) -> Self {
        Self { merchant: merchant, amount: amount, time: time }
    }
}

fn main() {
    let acc: Account = Account::new(true, 100);
    println!("active_card: {:?}, available_limit: {:?}", acc.active_card, acc.available_limit);
    let data: &str = r#"
        {
            "merchant": "Burguer King",
            "amount": 20,
            "time": "2022-07-08T09:10:11Z"
        }
    "#;
    let deserialization_result: Result<Transaction, serde_json::Error> = serde_json::from_str::<Transaction>(data);
    let res: Transaction = deserialization_result.expect("This should not panic");
    println!("merchant: {:?}, amount: {:?}, time: {:?}", res.merchant, res.amount, res.time);
}
