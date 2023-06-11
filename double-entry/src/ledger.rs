use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error;

const INTERCHANGE_FEE: Decimal = dec!(0.02);

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct AccountInfo {
    amount: Decimal,
    version: u32,
    off_balance: bool,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BookAccount {
    AssetSettled,
    AssetCurrentLimit,
    AssetMaxCurrentLimit,
    AssetTransitoryBank,
    LiabilityPayable,
    LiabilityReceivable,
    LiabilityCurrentLimitCp,
    LiabilityMaxCurrentLimitCp,
    EquityInterchange,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    id: Uuid,
    amount: Decimal, // always positive
    debit_account: BookAccount,
    credit_account: BookAccount,
    post_date: OffsetDateTime, // the day the entry actually ocurred
    merchant: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CardIssued;

impl CardIssued {
    pub fn new(max_limit: Decimal) -> Vec<Entry> {
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
}

#[derive(Debug, Eq, PartialEq)]
pub struct Purchase;

impl Purchase {
    pub fn new(merchant: String, amount: Decimal) -> Vec<Entry> {
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
}

#[derive(Debug, Eq, PartialEq)]
pub struct ClosedBill;

impl ClosedBill {
    pub fn new(closed_amount: Decimal) -> Vec<Entry> {
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
}

#[derive(Debug, Eq, PartialEq)]
pub struct Payment;

impl Payment {
    pub fn new(payment_amount: Decimal) -> Vec<Entry> {
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
}

#[derive(Debug, PartialEq, Eq)]
pub enum CardStatus {
    NotIssued,
    Inactive,
    Active,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ledger {
    card: Card,
    pub accounts: HashMap<BookAccount, AccountInfo>,
    pub journal: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    status: CardStatus,
    max_limit: Decimal,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            card: Card {
                status: CardStatus::NotIssued,
                max_limit: dec!(0.00),
            },
            journal: vec![],
            accounts: HashMap::from([
                (
                    BookAccount::AssetSettled,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: false,
                    },
                ),
                (
                    BookAccount::LiabilityPayable,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: false,
                    },
                ),
                (
                    BookAccount::EquityInterchange,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: false,
                    },
                ),
                
                (
                    BookAccount::AssetCurrentLimit,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: true,
                    },
                ),
                (
                    BookAccount::LiabilityCurrentLimitCp,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: true,
                    },
                ),
                (
                    BookAccount::AssetMaxCurrentLimit,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: true,
                    },
                ),
                (
                    BookAccount::LiabilityMaxCurrentLimitCp,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: true,
                    },
                ),
                (
                    BookAccount::LiabilityReceivable,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: false,
                    },
                ),
                (
                    BookAccount::AssetTransitoryBank,
                    AccountInfo {
                        amount: dec!(0.00),
                        version: 0,
                        off_balance: false,
                    },
                ),
            ]),
        }
    }

    pub fn get_balance(&self) -> Decimal {
        self.accounts.get(&BookAccount::AssetCurrentLimit).unwrap().amount.abs()
    }

    pub fn process(&mut self, entries: Vec<Entry>) -> Result<(), error::Result> {
        for entry in entries {
            // update book accounts
            match self.accounts.get_mut(&entry.debit_account) {
                Some(debit_account) => {
                    *debit_account = AccountInfo {
                        amount: debit_account.amount - entry.amount, // subtract from debit account
                        version: debit_account.version + 1,
                        off_balance: debit_account.off_balance,
                    }
                }
                None => return Err(error::Result::BookAccountNonExistent),
            };
            match self.accounts.get_mut(&entry.credit_account) {
                Some(credit_account) => {
                    *credit_account = AccountInfo {
                        amount: credit_account.amount + entry.amount, // add to credit account
                        version: credit_account.version + 1,
                        off_balance: credit_account.off_balance,
                    }
                }
                None => return Err(error::Result::BookAccountNonExistent),
            };

            // update journal
            self.journal.push(entry);
        }
        Ok(())
    }

    pub fn issue_card(&mut self, max_limit: Decimal) -> Result<CardStatus, error::Result> {
        match &self.card.status {
            CardStatus::NotIssued => {
                let card = Card {
                    status: CardStatus::Inactive,
                    max_limit: max_limit.to_owned(),
                };
                self.card = card;

                let entries = CardIssued::new(max_limit);
                self.process(entries)?;

                Ok(CardStatus::Inactive)
            }
            _ => Err(error::Result::CardAlreadyIssued),
        }
    }

    pub fn activate_card(&mut self) -> Result<CardStatus, error::Result> {
        match &self.card.status {
            CardStatus::NotIssued => Err(error::Result::CardNotIssued),
            _ => {
                self.card.status = CardStatus::Active;
                Ok(CardStatus::Active)
            }
        }
    }

    pub fn process_purchase(
        &mut self,
        merchant: String,
        amount: Decimal,
    ) -> Result<(), error::Result> {
        match &self.card.status {
            CardStatus::Active => {
                let balance = self.get_balance();
                if balance < amount {
                    return Err(error::Result::InsufficientLimit);
                }

                let entries = Purchase::new(merchant, amount);
                self.process(entries)?;

                Ok(())
            }
            CardStatus::NotIssued => Err(error::Result::CardNotIssued),
            CardStatus::Inactive => Err(error::Result::CardInactive),
        }
    }

    pub fn close_bill(&mut self) -> Result<(), error::Result> {
        match &self.card.status {
            CardStatus::NotIssued => Err(error::Result::CardNotIssued),
            _ => {
                match self.accounts.get(&BookAccount::AssetSettled) {
                    Some(acc) => {
                        let bill_amount = acc.amount.abs();
                        let entries = ClosedBill::new(bill_amount);
                        self.process(entries)?;
                    }
                    None => return Err(error::Result::BookAccountNonExistent),
                }

                Ok(())
            }
        }
    }

    pub fn process_payment(&mut self, payment_amount: Decimal) -> Result<(), error::Result> {
        match &self.card.status {
            CardStatus::NotIssued => Err(error::Result::CardNotIssued),
            _ => {
                let entries = Payment::new(payment_amount);
                self.process(entries)?;

                Ok(())
            }
        }
    }
}
