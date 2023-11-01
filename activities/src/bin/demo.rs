// fn main() {
//     let age = 15;
//     if age >= 21 {
//         println!("valid age")
//     } else {
//         println!("invalid age")
//     }
// }

fn main() {
    let mut i = 3;
    loop {
        if i == 0 {
            break;
        }
        println!("{:?}", i);
        i-=1;
    }
    println!("done");
}