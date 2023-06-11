use rust_decimal_macros::dec;

mod error;
mod ledger;
mod movement;

fn main() -> Result<(), error::Result> {
    let mut ledger = ledger::Ledger::new();
    ledger.issue_card(dec!(1000.00))?;
    ledger.activate_card()?;
    ledger.process_purchase("Burguer King".to_string(), dec!(20.00))?;
    ledger.close_bill()?;
    ledger.process_payment(dec!(20.00))?;

    println!(
        "card: {:#?}, journal: {:#?}, book accounts: {:#?}",
        ledger.get_balance(),
        ledger.journal,
        ledger.accounts
    );
    Ok(())
}
