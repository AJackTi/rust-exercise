use serde_derive::*;
mod error;
pub use error::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    // Err("No transactions".to_string())
    // Ok(Vec::new())
    // let s = std::fs::read_to_string(fname).unwrap();
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    // let t: Vec<Transaction> = serde_json::from_str(&s).unwrap();
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    Ok(t)
}

pub fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // std::fs
    //     ::read_to_string(fname)
    //     .map_err(|e| e.into())
    //     .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))

    // Ok(match
    //     serde_json::from_str(
    //         &(match std::fs::read_to_string(fname) {
    //             Ok(v) => v,
    //             Err(e) => {
    //                 return Err(e.into());
    //             }
    //         })
    //     )
    // {
    //     Ok(v) => v,
    //     Err(e) => {
    //         return Err(e.into());
    //     }
    // })
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    let trans = get_transactions_b(fname)?;
    for t in trans {
        if t.from == uname {
            return Ok(t);
        }
    }

    Err(TransactionError::Mess("Could not find transaction with that name").into())
}
