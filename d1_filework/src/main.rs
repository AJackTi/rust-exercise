use serde_derive::*;

#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    Mess(&'static str),
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

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

fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
    let trans = get_transactions_b(fname).ok()?;
    for t in trans {
        if t.from == uname {
            return Some(t);
        }
    }

    None
}
