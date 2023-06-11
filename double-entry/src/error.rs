use std::fmt;

#[derive(Debug)]
pub enum Result {
    InsufficientLimit,
    CardAlreadyIssued,
    CardNotIssued,
    CardInactive,
    BookAccountNonExistent,
    // DoubleTransaction,
    // HighFrequencySmallInterval,
}

impl std::error::Error for Result {}
impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Result::InsufficientLimit => write!(f, "insufficient_limit"),
            Result::CardAlreadyIssued => write!(f, "card_already_issued"),
            Result::CardNotIssued => write!(f, "card_not_issued"),
            Result::CardInactive => write!(f, "card_inactive"),
            Result::BookAccountNonExistent => write!(f, "book_acount_nonexistent"),
            // Result::DoubleTransaction => write!(f, "doubled_transaction"),
            // Result::HighFrequencySmallInterval => write!(f, "high-frequency-small-interval"),
        }
    }
}
