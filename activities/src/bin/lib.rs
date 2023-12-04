mod error;
use serde_derive::*;
use crate::error::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => {
            return Err(e.into());
        }
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => {
            return Err(e.into());
        }
    };
    Ok(t)

    // Ok(Vec::new())
    // Err("No Trans".to_string())
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
    let trans = get_transactions(fname).ok()?;
    for t in trans {
        if t.from == uname {
            return Some(t);
        }
    }

    None
}

pub fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // std::fs
    //     ::read_to_string(fname)
    //     .map_err(|e| e.into())
    //     .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))

    // Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)

    Ok(match
        serde_json::from_str(
            &(match std::fs::read_to_string(fname) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e.into());
                }
            })
        )
    {
        Ok(v) => v,
        Err(e) => {
            return Err(e.into());
        }
    })
}
