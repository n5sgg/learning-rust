
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::ledger::{Entry, BookAccount};

const INTERCHANGE_FEE: Decimal = dec!(0.02);

pub fn card_issued(max_limit: Decimal) -> Vec<Entry> {
    let now = OffsetDateTime::now_utc();
    let id = Uuid::new_v4();

    vec![
        Entry {
            id,
            debit_account: BookAccount::AssetMaxCurrentLimit,
            credit_account: BookAccount::LiabilityMaxCurrentLimitCp,
            amount: max_limit,
            post_date: now,
            merchant: None,
        },
        Entry {
            id,
            debit_account: BookAccount::AssetCurrentLimit,
            credit_account: BookAccount::LiabilityCurrentLimitCp,
            amount: max_limit,
            post_date: now,
            merchant: None,
        },
    ]
}

pub fn purchase(merchant: String, amount: Decimal) -> Vec<Entry> {
    let now = OffsetDateTime::now_utc();
    let id = Uuid::new_v4();
    let interchange: Decimal = (amount * INTERCHANGE_FEE).round_dp(2);

    vec![
        Entry {
            id,
            debit_account: BookAccount::AssetSettled,
            credit_account: BookAccount::LiabilityPayable,
            amount: amount,
            post_date: now,
            merchant: Some(merchant.to_string()),
        },
        Entry {
            id,
            debit_account: BookAccount::LiabilityPayable,
            credit_account: BookAccount::EquityInterchange,
            amount: interchange,
            post_date: now,
            merchant: Some(merchant.to_string()),
        },
        Entry {
            id,
            debit_account: BookAccount::LiabilityCurrentLimitCp,
            credit_account: BookAccount::AssetCurrentLimit,
            amount: amount,
            post_date: now,
            merchant: Some(merchant.to_string()),
        },
    ]
}

pub fn closed_bill(closed_amount: Decimal) -> Vec<Entry> {
    let now = OffsetDateTime::now_utc();
    let id = Uuid::new_v4();

    vec![Entry {
        id,
        debit_account: BookAccount::LiabilityReceivable,
        credit_account: BookAccount::AssetSettled,
        amount: closed_amount,
        post_date: now,
        merchant: None,
    }]
}

pub fn payment(payment_amount: Decimal) -> Vec<Entry> {
    let now = OffsetDateTime::now_utc();
    let id = Uuid::new_v4();

    vec![
        Entry {
            id,
            debit_account: BookAccount::AssetCurrentLimit,
            credit_account: BookAccount::LiabilityCurrentLimitCp,
            amount: payment_amount,
            post_date: now,
            merchant: None,
        },
        Entry {
            id,
            debit_account: BookAccount::AssetTransitoryBank,
            credit_account: BookAccount::LiabilityReceivable,
            amount: payment_amount,
            post_date: now,
            merchant: None,
        },
    ]
}
