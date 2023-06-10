use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rusty_money::{iso, Money};

const INTERCHANGE_FEE: Decimal = dec!(0.02);

fn get_currency(currency_code: String) -> &'static rusty_money::iso::Currency {
    match iso::find(&currency_code) {
        Some(n) => n,
        None => rusty_money::iso::USD,
    }
}

fn main() {
    let currency_code = get_currency("USD".to_string());

    let amount = Decimal::new(10000, 2); // $100
    let interchange: Decimal = (amount * INTERCHANGE_FEE).round_dp(2); // 100 * 2% = $2
    let payable = amount - interchange; // 100 - 2 = $98

    println!(
        "total: {}, payable: {}, interchange_fee: {}",
        Money::from_decimal(amount, currency_code),
        Money::from_decimal(payable, currency_code),
        Money::from_decimal(interchange, currency_code)
    );
}
