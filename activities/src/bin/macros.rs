#![macro_escape]

#[macro_export]
macro_rules! welcome {
    () => {
        println!("Welcome to RUST Macro", );
    };
}

fn main() {}
