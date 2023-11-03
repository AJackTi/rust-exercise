// fn main() {
//     let age = 15;
//     if age >= 21 {
//         println!("valid age")
//     } else {
//         println!("invalid age")
//     }
// }

// fn main() {
//     let mut i = 3;
//     loop {
//         if i == 0 {
//             break;
//         }
//         println!("{:?}", i);
//         i-=1;
//     }
//     println!("done");
// }

// fn main() {
//     let mut i = 1;
//     while i <=3 {
//         println!("{:?}", i);
//         i += 1;
//     }
// }

// enum Direction {
//     Left,
//     Right,
//     Up
// }

// fn main() {
//     let go = Direction::Left;
//     match go {
//         Direction::Left => println!("go left"),
//         Direction::Right => println!("go right"),
//         Direction::Up => println!("go up"),
//     }
// }

// struct GroceryItem {
//     stock: i32,
//     price: f64,
// }

// fn main() {
//     let cereal = GroceryItem {
//         stock: 10,
//         price: 2.99,
//     };

//     println!("stock {:?}", cereal.stock);
//     println!("price {:?}", cereal.price);
// }

// fn main() {
//     let coord = (2,3);
//     println!("{:?} {:?}", coord.0, coord.1);

//     let (x,y) = (2,3);
//     println!("{:?} {:?}", x, y);

//     let (name, age) = ("Emma", 20);
//     println!("{:?} {:?}", name, age);

//     let favorites = ("red", 14, "TX", "pizza", "TV SHOW", "home");

//     let state = favorites.2;
//     let place = favorites.5;
//     print!("state: {:?}", state);
//     print!("place: {:?}", place);
// }

// enum Access {
//     Admin,
//     Manager,
//     User,
//     Guest,
// }

// fn main() {
//     // secret file: admins only
//     let access_level = Access::Guest;
//     let can_access_file = match access_level {
//         Access::Admin => true,
//         _=> false,
//     };

//     println!("can access: {:?}", can_access_file);
// }

// struct Book {
//     pages: i32,
//     rating: i32,
// }

// fn display_page_count(book: &Book) {
//     println!("pages = {:?}", book.pages);
// }

// fn display_rating(book: &Book) {
//     println!("rating = {:?}", book.rating);
// }

// fn main() {
//     let book = Book {
//         pages:5,
//         rating: 9,
//     };

//     display_page_count(&book);
//     display_rating(&book);
// }

// struct Temperature {
//     degrees_f: f64,
// }

// impl Temperature {
//     fn freezing() -> Self {
//         Self {
//             degrees_f: 32.0
//         }
//     }

//     fn boiling() -> Self {
//         Self {
//             degrees_f: 212.0
//         }
//     }

//     fn show_temp(&self) {
//         println!("{:?} degrees F", self.degrees_f);
//     }
// }

// fn main() {
//     let hot = Temperature {
//         degrees_f: 99.9
//     };

//     hot.show_temp();
//     // Temperature::show_temp(&hot);

//     let cold = Temperature::freezing();
//     cold.show_temp();

//     let boiling = Temperature::boiling();
//     boiling.show_temp()
// }

// struct Test {
//     score: i32,
// }

// fn main() {
//     let my_scores = vec![
//         Test{score: 90},
//         Test{score: 91},
//         Test{score: 92},
//         Test{score: 93},
//     ];

//     for test in my_scores {
//         println!("score = {:?}", test.score);
//     }
// }

// struct LineItem {
//     name: String,
//     count: i32,
// }

// fn print_name(name: &str) {
//     println!("name: {:?}", name);
// }

// fn main() {
//     let receipt = vec![
//         LineItem {
//             name:"cereal".to_owned(),
//             count: 1
//         },
//         LineItem {
//             name: String::from("fruit"),
//             count: 3
//         }
//     ];

//     for item in receipt {
//         print_name(&item.name);
//         println!("count: {:?}", item.count);
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum Position {
//     Manager,
//     Supervisor,
//     Worker
// }

// #[derive(Debug, Clone, Copy)]
// struct Employee {
//     position: Position,
//     work_hours: i64,
// }

// fn print_employee(emp: Employee) {
//     println!("{:?}", emp);
// }

// fn main() {
//     let me = Employee {
//         position: Position::Worker,
//         work_hours: 40
//     };

//     // match me.position {
//     //     Position::Manager => println!("manager"),
//     //     Position::Supervisor => println!("supervisor"),
//     //     Position::Worker => println!("worker"),
//     // }
//     println!("{:?}", me.position);
//     println!("{:?}", me);
//     print_employee(me);
//     print_employee(me);
// }

// enum Discount {
//     Percent(i32),
//     Flat(i32),
// }

// struct Ticket {
//     event: String,
//     price: i32,
// }

// fn main() {
//     let n = 3;
//     match n {
//         3 => println!("three"),
//         other => println!("number: {:?}", other),
//     }

//     let flat = Discount::Flat(2);
//     match flat {
//         Discount::Flat(2) => println!("flat 2"),
//         Discount::Flat(amount) => println!("flat discount of {:?}", amount),
//         _ => (),
//     }

//     let concert = Ticket {
//         event: "concert".to_owned(),
//         price: 50,
//     };
//     match concert {
//         Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
//         Ticket { price, .. } => println!("price = {:?}", price),
//     }
// }

// struct Survey {
//     q1: Option<i32>,
//     q2: Option<bool>,
//     q3: Option<String>,
// }

// fn main() {
//     let response = Survey {
//         q1: None,
//         q2: Some(true),
//         q3: Some("A".to_owned()),
//     };

//     match response.q1 {
//         Some(ans) => println!("q1: {:?}", ans),
//         None => println!("q1: no response"),
//     }

//     match response.q2 {
//         Some(ans) => println!("q2: {:?}", ans),
//         None => println!("q2: no response"),
//     }

//     match response.q3 {
//         Some(ans) => println!("q3: {:?}", ans),
//         None => println!("q3: no response"),
//     }
// }

enum Color {
    Red,
    Blue,
}

/// A piece of mail
struct Mail {
    /// The destination address.
    address: String,
}

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {}
