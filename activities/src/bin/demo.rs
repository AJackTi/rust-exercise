//     let age = 15;
// fn main() {
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

// enum Color {
//     Red,
//     Blue,
// }

// /// A piece of mail
// struct Mail {
//     /// The destination address.
//     address: String,
// }

// /// Adds two numbers together.
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {}

// fn main() {
//     let numbers = vec![1, 2, 3];
//     match numbers.is_empty() {
//         true => println!("no numbers"),
//         false => println!("has numbers"),
//     }
// }

// #[derive(Debug)]
// enum MenuChoice {
//     MainMenu,
//     Start,
//     Quit,
// }

// fn get_choice(input: &str) -> Result<MenuChoice, String> {
//     match input {
//         "mainmenu" => Ok(MenuChoice::MainMenu),
//         "start" => Ok(MenuChoice::Start),
//         "quit" => Ok(MenuChoice::Quit),
//         _ => Err("menu choice not found".to_owned()),
//     }
// }

// fn print_choice(choice: &MenuChoice) {
//     println!("choice = {:?}", choice);
// }

// fn pick_choice(input: &str) -> Result<(), String> {
//     let choice = get_choice(input)?;
//     print_choice(&choice);
//     Ok(())
// }

// fn main() {
//     let choice = pick_choice("end");
//     println!("choice value = {:?}", choice);

//     // pick_choice("start");

//     // let choice: Result<MenuChoice, String> = get_choice("mainmenu");
//     // match choice {
//     //     Ok(inner_choice) => print_choice(&inner_choice),
//     //     Err(e) => println!("error = {:?}", e),
//     // }

//     // print_choice(&choice.unwrap());
//     // println!("choice = {:?}", choice);
// }

// use std::collections::HashMap;

// #[derive(Debug)]
// struct Contents {
//     content: String,
// }

// fn main() {
//     let mut lockers = HashMap::new();
//     lockers.insert(1, Contents { content: "stuff".to_owned() });
//     lockers.insert(2, Contents { content: "shift".to_owned() });
//     lockers.insert(3, Contents { content: "gym shorts".to_owned() });

//     for (locker_number, content) in lockers.iter() {
//         println!("number: {}, content: is {:?}", locker_number, content);
//     }
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {
//     let sum = add(1, 2);
//     println!("sum = {}", sum);

//     let add = |a: i32, b: i32| -> i32 { a + b };

//     let sum = add(1, 3);
//     println!("sum = {}", sum);

//     let add = |a, b| a + b;

//     let sum = add(1, 4);
//     println!("sum = {}", sum);
// }

// fn maybe_num() -> Option<i32> {
//     None
// }

// fn maybe_word() -> Option<String> {
//     None
// }

// fn main() {
//     // let plus_one = match maybe_num() {
//     //     Some(num) => Some(num + 1),
//     //     None => None,
//     // };

//     // let plus_one = maybe_num().map(|num| num + 1);

//     let word_length = maybe_word()
//         .map(|word| word.len())
//         .map(|len| len * 2);
// }

// fn main() {
//     let a: Option<i32> = Some(1);
//     dbg!(a);

//     let a_is_some = a.is_some();
//     dbg!(a_is_some);

//     let a_is_none = a.is_none();
//     dbg!(a_is_none);

//     let a_mapped = a.map(|num| num + 1);
//     dbg!(a_mapped);

//     let a_filtered = a.filter(|num| num == &1);
//     dbg!(a_filtered);

//     let a_or_else = a.or_else(|| Some(5));
//     dbg!(a_or_else);

//     let unwrapped = a.unwrap_or_else(|| 0);
//     dbg!(unwrapped);
// }

// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];

//     let mut plus_one = vec![];
//     for num in &numbers {
//         plus_one.push(num + 1);
//     }

//     let plus_one: Vec<_> = numbers
//         .iter()
//         .map(|num| num + 1)
//         .collect();
//     dbg!(plus_one);

//     let new_numbers: Vec<_> = numbers
//         .iter()
//         .filter(|num| num <= &&3)
//         .collect();
//     dbg!(new_numbers);

//     let find_me: Option<&i32> = numbers.iter().find(|num| num == &&40);
//     dbg!(find_me);

//     let count = numbers.iter().count();
//     dbg!(count);

//     let last = numbers.iter().last();
//     dbg!(last);

//     let min = numbers.iter().min();
//     dbg!(min);

//     let max = numbers.iter().max();
//     dbg!(max);

//     let take: Vec<_> = numbers.iter().take(3).collect();
//     dbg!(take);
// }

// fn main() {
//     let range = 1..=3;
//     for num in range {
//         println!("num = {}", num);
//     }

//     println!("");

//     let range = 1..4;
//     for num in range {
//         println!("num = {}", num);
//     }

//     println!("");

//     for num in 1..4 {
//         println!("{}", num);
//     }

//     println!("");

//     for ch in 'a'..='f' {
//         println!("{}", ch);
//     }
// }

// enum Color {
//     Red,
//     Blue,
//     Green,
// }

// fn main() {
//     let maybe_user = Some("Jerry");
//     match maybe_user {
//         Some(user) => println!("user = {:?}", user),
//         None => println!("no user"),
//     }

//     if let Some(user) = maybe_user {
//         println!("user = {}", user);
//     } else {
//         println!("no user");
//     }

//     let red = Color::Red;
//     if let Color::Red = red {
//         println!("its red");
//     } else {
//         println!("its not red");
//     }
// }

// fn main() {
//     let mut data = Some(3);
//     while let Some(i) = data {
//         println!("loop");
//         data = None;
//     }

//     let numbers = vec![1, 2, 3];
//     let mut numebr_iter = numbers.iter();
//     while let Some(num) = numebr_iter.next() {
//         println!("num = {:?}", num);
//     }

//     println!("done");
// }

// use crate::greet::hello;

mod greet {
    pub fn hello() {
        println!("hello");
    }

    pub(crate) fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn sub(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    use greet::*;
    hello();
    goodbye();

    math::add(1, 2);
}
