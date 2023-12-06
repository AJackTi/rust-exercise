extern crate d1_filework;

use d1_filework::{ TransactionError, get_transactions_b, get_first_transaction_for };

fn main() -> Result<(), TransactionError> {
    let trans = get_transactions_b("test_data/transactions.json").expect(
        "Could not load transactions"
    );

    for t in trans {
        println!("{:?}", t);
    }

    // First trans
    let t = get_first_transaction_for("test_data/transactions.json", "abc").ok_or(
        "could not get first transaction"
    )?;
    println!("{:?}", t);

    Ok(())
}
