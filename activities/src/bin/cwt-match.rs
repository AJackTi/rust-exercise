fn main() {
    let my_name = "AJackTi";
    match my_name {
        "AJackTi" => println!("that is my name"),
        "Bod" => print!("not my name"),
        "Alice" => println!("hello alice"),
        _ => println!("nice to meet you")
    }
}