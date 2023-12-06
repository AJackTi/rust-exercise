use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn main() {
    let trans = get_transactions("test_data/transactions.json").expect(
        "Could not load transactions"
    );

    for t in trans {
        println!("{:?}", t);
    }
}

fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
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
