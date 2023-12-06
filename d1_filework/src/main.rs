extern crate d1_filework;

use d1_filework::{ get_transactions_b, get_first_transaction_for };

use failure::Error;

fn main() -> Result<(), Error> {
    let trans = get_transactions_b("test_data/transactions.json").expect(
        "Could not load transactions"
    );

    for t in trans {
        println!("{:?}", t);
    }

    // First trans
    let t = get_first_transaction_for("test_data/transactions.json", "abc");
    match t {
        Ok(t) => println!("Found transaction: {:?}", t),
        Err(e) => println!("Error {:?}, Backtrace: {:?}", e, e.backtrace()),
    }

    Ok(())
}
