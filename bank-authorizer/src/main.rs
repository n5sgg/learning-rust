use serde::{Deserialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

// #[serde_as]
// #[derive(Deserialize)]
// struct Account {
//     active_card: bool
//     available_limit: u32
// }

#[derive(Debug)]
#[serde_as]
#[derive(Deserialize)]
struct Transaction {
    merchant: String,
    amount: u32,
    #[serde_as(as = "Rfc3339")]
    time: OffsetDateTime,
}

fn main() {
    let data = r#"
        {
            "merchant": "Burguer King",
            "amount": 20,
            "time": "2022-07-08T09:10:11Z"
        }
    "#;
    let deserialization_result = serde_json::from_str::<Transaction>(data);
    let res = deserialization_result.expect("This should not panic");
    println!("merchant: {:?}, amount: {:?}, time: {:?}", res.merchant, res.amount, res.time);
}
