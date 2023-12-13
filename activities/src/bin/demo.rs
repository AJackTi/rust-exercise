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

// mod greet {
//     pub fn hello() {
//         println!("hello");
//     }

//     pub(crate) fn goodbye() {
//         println!("goodbye");
//     }
// }

// mod math {
//     pub fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }
//     fn sub(a: i32, b: i32) -> i32 {
//         a * b
//     }
// }

// fn main() {
//     use greet::*;
//     hello();
//     goodbye();

//     math::add(1, 2);
// }

// fn all_caps(word: &str) -> String {
//     word.to_uppercase()
// }

// fn main() {}

// #[cfg(test)]
// mod test {
//     use crate::all_caps;

//     #[test]
//     fn check_all_caps() {
//         let result = all_caps("hello");
//         let expected = String::from("HELLO");
//         assert_eq!(result, expected, "string should be all uppercase");
//     }
// }

// use std::io;

// fn get_input() -> io::Result<String> {
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer)?;
//     Ok(buffer.trim().to_owned())
// }
// fn main() {
//     let mut all_input = vec![];
//     let mut times_input = 0;
//     while times_input < 2 {
//         match get_input() {
//             Ok(words) => {
//                 all_input.push(words);
//                 times_input += 1;
//             }
//             Err(err) => println!("error = {}", err),
//         }
//     }

//     for input in all_input {
//         println!("Original: {}, capitalized: {}", input, input.to_uppercase());
//     }
// }

// trait Fall {
//     fn hit_ground(&self);
// }

// struct Vase;
// impl Fall for Vase {
//     fn hit_ground(&self) {
//         println!("the vase broke!");
//     }
// }

// struct Cat;
// impl Fall for Cat {
//     fn hit_ground(&self) {
//         println!("the cat usually walked away");
//     }
// }

// fn fall(thing: impl Fall) {
//     thing.hit_ground()
// }

// fn main() {
//     fall(Vase {});
//     fall(Cat {});
// }

// #[derive(Debug)]
// struct Package {
//     weight: f64,
// }

// impl Package {
//     fn new(weight: f64) -> Self {
//         Self { weight }
//     }
// }

// impl Default for Package {
//     fn default() -> Self {
//         Self { weight: 3.0 }
//     }
// }

// fn main() {
//     let p = Package::default();
//     dbg!(p);
// }

// struct Dimensions {
//     width: f64,
//     height: f64,
//     depth: f64,
// }

// struct ConveyorBelt<T: Convey> {
//     pub items: Vec<T>,
// }

// impl<T: Convey> ConveyorBelt<T> {
//     pub fn add(&mut self, item: T) {
//         self.items.push(item);
//     }
// }

// struct CarPart {
//     width: f64,
//     height: f64,
//     depth: f64,
//     weight: f64,
//     part_number: String,
// }

// impl Default for CarPart {
//     fn default() -> Self {
//         Self { width: 5.0, height: 1.0, depth: 2.0, weight: 3.0, part_number: "abc".to_owned() }
//     }
// }

// trait Convey {
//     fn weight(&self) -> f64;
//     fn dimensions(&self) -> Dimensions;
// }

// impl Convey for CarPart {
//     fn weight(&self) -> f64 {
//         self.weight
//     }

//     fn dimensions(&self) -> Dimensions {
//         Dimensions { width: self.width, height: self.height, depth: self.depth }
//     }
// }

// fn main() {
//     // ok
//     let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![] };
//     belt.add(CarPart::default());

//     // error
//     // let mut belt = ConveyorBelt { items: vec![] };
//     // belt.add(5)
// }

// trait Sale {
//     fn amount(&self) -> f64;
// }

// struct FullSale(f64);
// impl Sale for FullSale {
//     fn amount(&self) -> f64 {
//         self.0
//     }
// }

// struct OneDollarOffCoupon(f64);
// impl Sale for OneDollarOffCoupon {
//     fn amount(&self) -> f64 {
//         self.0 - 1.0
//     }
// }

// struct TenPercentOffPromo(f64);
// impl Sale for TenPercentOffPromo {
//     fn amount(&self) -> f64 {
//         self.0 * 0.9
//     }
// }

// fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
//     sales
//         .iter()
//         .map(|sale| sale.amount())
//         .sum()
// }

// fn main() {
//     let price = 20.0;
//     let regular = Box::new(FullSale(price));
//     let coupon = Box::new(OneDollarOffCoupon(price));
//     let promo = Box::new(TenPercentOffPromo(price));

//     let sales: Vec<Box<dyn Sale>> = vec![regular, coupon, promo];
//     let revenue = calculate_revenue(&sales);
//     println!("revenue = {}", revenue);
// }

// #[derive(Debug)]
// struct Cards {
//     inner: Vec<IdCard>,
// }

// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
// enum City {
//     Barland,
//     Bazopolis,
//     Fooville,
// }

// #[derive(Debug)]
// struct IdCard {
//     name: String,
//     age: u8,
//     city: City,
// }

// impl IdCard {
//     pub fn new(name: &str, age: u8, city: City) -> Self {
//         Self { name: name.to_string(), age, city }
//     }
// }

// fn new_ids() -> Cards {
//     Cards {
//         inner: vec![
//             IdCard::new("Amy", 1, City::Fooville),
//             IdCard::new("Matt", 10, City::Barland),
//             IdCard::new("Bailee", 20, City::Barland),
//             IdCard::new("Anthony", 30, City::Bazopolis),
//             IdCard::new("Tina", 40, City::Bazopolis)
//         ],
//     }
// }

// #[derive(Debug)]
// struct YoungPeople<'a> {
//     inner: Vec<&'a IdCard>,
// }

// impl<'a> YoungPeople<'a> {
//     fn living_in_fooville(&self) -> Self {
//         Self {
//             inner: self.inner
//                 .iter()
//                 .filter(|id| id.city == City::Fooville)
//                 .map(|id| *id)
//                 .collect(),
//         }
//     }
// }

// fn main() {
//     let ids = new_ids();
//     let young = YoungPeople {
//         inner: ids.inner
//             .iter()
//             .filter(|id| id.age <= 20)
//             .collect(),
//     };

//     println!("ids");
//     for id in ids.inner.iter() {
//         println!("{:?}", id);
//     }

//     println!("\nyoung");
//     for id in young.inner.iter() {
//         println!("{:?}", id);
//     }

//     println!("\nliving in fooville");
//     for id in young.living_in_fooville().inner.iter() {
//         println!("{:?}", id);
//     }
// }

// mod macros;

// fn main() {
//     welcome!()
// }

// extern crate mylib;

// use mylib::demo;

// fn main() {
//     demo()
// }

// fn main() {
//     println!("highest: {}", highest(1, 2, 3));
// }

// fn highest(a: i32, b: u32, c: i8) -> i32 {
//     let mut res = a;
//     if (b as i32) > res {
//         res = b as i32;
//     }

//     if (c as i32) > res {
//         res = c as i32;
//     }

//     res
// }

// const MAX_SPEED: i32 = 9000;

// fn clamp_speed(speed: i32) -> i32 {
//     if speed > MAX_SPEED { MAX_SPEED } else { speed }
// }

// fn main() {}

// #[derive(Debug, Clone, Copy)]
// struct NeverZero(i32);

// impl NeverZero {
//     fn new(i: i32) -> Result<Self, String> {
//         if i == 0 { Err("cannot be zero".to_owned()) } else { Ok(Self(i)) }
//     }
// }

// fn divide(a: i32, b: NeverZero) -> i32 {
//     let b = b.0;
//     a / b
// }

// fn main() {
//     match NeverZero::new(5) {
//         Ok(nz) => println!("{}", divide(10, nz)),
//         Err(e) => println!("{:?}", e),
//     }
// }

// fn say_hello(name: &mut &str) {
//     *name = "Alex";
//     println!("Hello {}", name);
// }

// fn main() {
//     let mut name = "John";
//     say_hello(&mut name);
//     println!("{}", name);
// }

// fn main() {
//     // let s = String::from("Hello");
//     // takes_ownership(&s);
//     // println!("{}", s);

//     let mut s = String::from("Hello");
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;
//     r3.push_str(" World");
//     println!("{}", r3);
//     println!("{}", s);
// }

// fn takes_ownership(s: &String) {
//     println!("s = {}", s);
// }

// fn main() {
//     let a = String::from("Hello World");
//     let r1 = &a[0..5];
//     println!("{}", r1);
//     println!("{}", a);
// }

// extern crate rand;

// use rand::Rng;
// use std::io;

// fn main() {
//     println!("Guess a number");
//     let secret = rand::thread_rng().gen_range(1, 10);
//     loop {
//         println!("Input your guess");
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("Fail");
//         let guess: i32 = guess.trim().parse().expect("Fail");
//         if guess == secret {
//             println!("Guessed correctly");
//             break;
//         } else {
//             println!("Try again");
//             if guess > secret {
//                 println!("You have guessed a higher no");
//             } else {
//                 println!("Value is smaller");
//             }
//         }
//     }
// }

// mod network {
//     fn connect() {}

//     pub mod server {
//         pub fn connect() {}
//     }
// }

// fn main() {
//     network::server::connect()
// }

// fn main() {
//     // let mut v = vec![1, 2, 3, 4];
//     // for i in &mut v {
//     //     *i *= 2;
//     //     println!("{}", i);
//     // }

//     let s1 = String::from("Hello");
//     let n = &s1[0..1];
//     println!("{}", n);
// }

// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn read() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => {
//             return Err(e);
//         }
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn main() {
//     let output = read();
//     match output {
//         Ok(fi) => println!("{}", fi),
//         Err(e) => println!("{}", e),
//     }
// }

// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn read() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;

//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn main() {
//     let output = read();
//     match output {
//         Ok(fi) => println!("{}", fi),
//         Err(e) => println!("{}", e),
//     }
// }

// fn change(s: &mut String) {
//     s.push_str(", World");
// }

// fn main() {
//     let mut s = String::from("Hello");
//     let r1 = &mut s;
//     println!("r1 = {}", r1);
//     let r2 = &mut s;
//     println!("r2 = {}", r2);
// }

#![allow(unused_assignments)]
#![allow(unused_variables)]

// macro_rules! my_macro {
//     () => {
//         println!("First macro");
//     };
// }

// macro_rules! name {
//     ($name:expr) => {
//         println!("Hey {}", $name);
//     };
// }

// macro_rules! name {
//     ($($name:expr),*) => {
//         $(println!("Hey {}", $name);)*
//     };
// }

// macro_rules! xy {
//     (x => $e:expr) => {
//         println!("X is {}", $e);
//     };
//     (y => $e:expr) => {
//         println!("Y is {}", $e);
//     };
// }

// macro_rules! build_fn {
//     ($fn_name:ident) => {
//         fn $fn_name() {
//             println!("{:?} was called", stringify!($fn_name));
//         }
//     };
// }

// fn main() {
//     my_macro!();
//     name!("John");
//     name!("John", "Mary", "Carol");
//     xy!(x=>5);
//     xy!(y=>3*9);
//     build_fn!(hey);
//     hey();
// }

// fn main() {
//     let mut a = 5;
//     mutate_value(&mut a);
//     println!("{}", a);
// }

// fn mutate_value(num: &mut i32) {
//     *num = 3;
// }

// References Rules:
//  1. One mutable reference in a scope
//  2. Many immutable references
//  3. Mutable and immutable can not coexist
//  4. Data should not change when immutable references are in scope

// fn main() {
//     let mut heap_mut = vec![4, 5, 6];
//     let ref1 = &mut heap_mut;
//     let ref2 = &mut heap_mut; // Error
//     println!("The first reference is {:?} and the second one is {:?}", ref1, ref2);
// }

// fn main() {
//     let mut heap_num = vec![4, 5, 6];
//     let ref1 = &heap_num;
//     let ref2 = &heap_num;
//     println!("The first references is {:?} and the second one is {:?}", ref1, ref2);
// }

// fn main() {
//     let mut heap_num = vec![4, 5, 6];
//     let ref1 = &heap_num;
//     let ref2 = &heap_num;
//     let ref3 = &mut heap_num;
//     println!(
//         "Immutable reference are {:?} and {:?} and teh mutable reference is {:?}",
//         ref1,
//         ref2,
//         ref3
//     );
// }

// fn main() {
//     let mut heap_num = vec![4, 5, 6];
//     let ref1 = &heap_num;
//     let ref2 = &heap_num;
//     println!("Immutable references are {:?} and {:?}", ref1, ref2);

//     let ref3 = &mut heap_num;
//     println!("Mutable reference is {:?}", ref3);
// }

// fn main() {
//     let mut heap_num = vec![4, 5, 6];
//     heap_num.push(86); // work
//     let ref1 = &heap_num;
//     // heap_num.push(86); // not work
//     let ref2 = &heap_num;
//     println!("Immutable references are {:?} and {:?}", ref1, ref2);
//     heap_num.push(86); // work
// }

// extern crate rand;

// use rand::Rng;

// use std::ops::Add;

// #[derive(Debug)]
// pub struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Self;
//     fn add(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// impl Point {
//     fn random() -> Self {
//         let mut tr = rand::thread_rng();
//         Point {
//             x: tr.gen(),
//             y: tr.gen(),
//         }
//     }
// }

// fn main() {
//     let a = Point { x: 3, y: 5 };
//     let b = Point { x: 30, y: 50 };

//     let c = a + b;
//     println!("{:?}", c);

//     let d = Point::random();
//     println!("c = {:?}", d);
// }

// struct Dog {}
// struct Cat {}

// impl Animal for Dog {
//     fn make_noise(&self) -> &'static str {
//         "woof"
//     }
// }

// impl Animal for Cat {
//     fn make_noise(&self) -> &'static str {
//         "meow"
//     }
// }

// trait Animal {
//     fn make_noise(&self) -> &'static str;
// }

// fn get_animal(rand_number: f64) -> Box<dyn Animal> {
//     if rand_number < 1.0 { Box::new(Dog {}) } else { Box::new(Cat {}) }
// }

// fn main() {
//     println!("The animal says {}", get_animal(0.5).make_noise());
//     println!("The animal says {}", get_animal(1.0).make_noise());
// }

// trait Summable<T> {
//     fn sum(&self) -> T;
// }

// impl Summable<i32> for Vec<i32> {
//     fn sum(&self) -> i32 {
//         let mut sum = 0;
//         for i in self {
//             sum += *i;
//         }
//         sum
//     }
// }

// fn main() {
//     let a = vec![1, 2, 3, 4, 5];
//     println!("sum = {}", a.sum());
// }

// use std::ops::Add;

// #[derive(Debug)]
// struct Point {
//     x: f64,
//     y: f64,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 1.3, y: 4.6 };
//     let p2 = Point { x: 3.7, y: 1.4 };
//     let p3 = p1 + p2;
//     println!("{:?}", p3);
// }

// trait Duplicatable {
//     fn dupl(&self) -> String;
// }

// impl Duplicatable for String {
//     fn dupl(&self) -> String {
//         format!("{0}{0}", *self)
//     }
// }

// impl Duplicatable for i32 {
//     fn dupl(&self) -> String {
//         format!("{}", *self * 2)
//     }
// }

// fn duplicate<T: Duplicatable>(x: T) {
//     println!("{}", x.dupl());
// }

// fn duplicate(x: &dyn Duplicatable) {
//     println!("{}", x.dupl());
// }

// fn main() {
//     let a = 42;
//     let b = "Hi John ".to_string();
//     // duplicate(a);
//     // duplicate(b)

//     duplicate(&a);
//     duplicate(&b);
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
// }

// #[derive(Debug)]
// struct Dog<'l> {
//     name: String,
//     owner: &'l Person,
// }

// impl Person {
//     // fn get_name(&self) -> &String {
//     //     &self.name
//     // }

//     fn get_name<'l>(&'l self) -> &'l String {
//         &self.name
//     }
// }

// fn main() {
//     let p1 = Person { name: String::from("John") };
//     let d1 = Dog { name: String::from("Max"), owner: &p1 };

//     println!("{:?}", p1);
//     println!("{:?}", d1);

//     let mut a: &String;
//     {
//         // WRONG
//         let p2 = Person { name: String::from("Mary") };
//         // a = p2.get_name();

//         // WORK
//         a = p1.get_name();
//     }
//     println!("{}", a);
// }

// use std::rc::Rc;

// struct Car {
//     brand: Rc<String>,
// }

// impl Car {
//     fn new(brand: Rc<String>) -> Car {
//         Car { brand: brand }
//     }

//     fn drive(&self) {
//         println!("{} is driving", &self.brand);
//     }
// }

// fn main() {
//     let brand = Rc::new(String::from("BMW"));
//     println!("pointers: {}", Rc::strong_count(&brand));
//     {
//         let car = Car::new(brand.clone());
//         car.drive();
//         println!("pointers: {}", Rc::strong_count(&brand));
//     }
//     println!("My car is a {}", brand);
//     println!("pointers: {}", Rc::strong_count(&brand));
// }

// use std::{ fs::{ File, OpenOptions, remove_dir, remove_file }, io::{ Write, Read } };

// fn main() {
//     // let mut file = File::create("src/example.txt").expect("create failed");
//     // file.write_all("Hello World\n".as_bytes()).expect("write failed")

//     // let mut file = OpenOptions::new()
//     //     .append(true)
//     //     .open("src/example.txt")
//     //     .expect("cannot open file");
//     // file.write_all("Adding content to the file\n".as_bytes()).expect("write failed")

//     let mut file = File::open("src/example.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     println!("{}", contents);

//     remove_file("src/example.txt").expect("delete failed")
// }

// use std::fs::File;

// fn main() {
//     let f = File::open("main.jpg");
//     match f {
//         Ok(f) => {
//             println!("file found: {:?}", f);
//         }
//         Err(e) => {
//             println!("file not found: {:?}", e);
//         }
//     }

//     println!("Continuing on with execution");
//     divide(Some(1))
// }

// const ANSWER_TO_LIFE: i32 = 42;

// fn divide(x: Option<i32>) {
//     match x {
//         Some(0) => panic!("Cannot divide by 0"),
//         Some(x) => println!("result is: {}", ANSWER_TO_LIFE / x),
//         None => println!("None received, the answer is {}", ANSWER_TO_LIFE),
//     }
// }

// use std::{ io::{ self, Read }, fs::File };

// // fn read_username_from_file() -> Result<String, io::Error> {
// //     let f = File::open("username.txt");
// //     let mut f = match f {
// //         Ok(file) => file,
// //         Err(e) => {
// //             return Err(e);
// //         }
// //     };

// //     let mut s = String::new();
// //     match f.read_to_string(&mut s) {
// //         Ok(_) => Ok(s),
// //         Err(e) => Err(e),
// //     }
// // }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("src/username.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn main() {
//     let a = read_username_from_file();
//     println!("{:?}", a);
// }

// use std::{ thread::{ self, sleep }, time::Duration };

// fn main() {
//     let mut threads = vec![];
//     for i in 0..10 {
//         let th = thread::spawn(move || {
//             sleep(Duration::from_millis(i * 1000));
//             println!("new thread {}", i);
//         });
//         threads.push(th);
//     }

//     for t in threads {
//         let _ = t.join();
//     }

//     println!("Main thread");
// }

// use std::{ sync::mpsc, thread, time::Duration };

// const NUM_THREADS: usize = 20;

// fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
//     thread::spawn(move || {
//         println!("setting timer {}", d);
//         thread::sleep(Duration::from_secs(d as u64));
//         println!("sending {}", d);
//         tx.send(d).unwrap()
//     });
// }

// fn main() {
//     // let (tx, rx) = mpsc::channel();
//     // thread::spawn(move || {
//     //     tx.send(42).unwrap();
//     // });

//     // println!("received {}", rx.recv().unwrap());

//     let (tx, rx) = mpsc::channel();
//     for i in 0..NUM_THREADS {
//         start_thread(i, tx.clone());
//     }

//     for j in rx.iter().take(NUM_THREADS) {
//         println!("received {}", j);
//     }
// }

// use std::{ sync::{ Arc, Mutex }, thread };

// fn main() {
//     let c = Arc::new(Mutex::new(0));
//     let mut threads = vec![];

//     for i in 0..10 {
//         let c = Arc::clone(&c);
//         let t = thread::spawn(move || {
//             let mut num = c.lock().unwrap();
//             *num += 1;
//         });
//         threads.push(t);
//     }

//     for th in threads {
//         th.join().unwrap();
//     }

//     println!("Result {}", *c.lock().unwrap());
// }

// fn main() {
//     let c = Arc::new(Mutex::new(0));
//     let mut threads = vec![];

//     for i in 0..10 {
//         let c = Arc::clone(&c);
//         let t = thread::spawn(move || {
//             let mut num = c.lock().unwrap();
//             *num += 1;
//         });
//         threads.push(t);
//     }

//     for th in threads {
//         th.join().unwrap();
//     }

//     println!("Result {}", *c.lock().unwrap());
// }

// fn main() {
//     let a = 10;
//     {
//         let b = 20;

//         let c = max(&a, &b);

//         println!("{}", c);
//     }
// }

// fn max<'a: 'b, 'b>(x: &'a i32, y: &'b i32) -> &'b i32 {
//     if *x > *y { x } else { y }
// }

// A Cons List is a data structure that contains two elements: the value of the
// current item and the next item. The last item in the list contains only a value called Nil
// without a next item.
// A Cons List is produced by recursively calling the cons function
// use crate::List::{ Cons, Nil };

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn main() {
//     // Error
//     // let list = Cons(1, Cons(2, Cons(5, Nil)));
//     // Work
//     let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// use std::rc::Rc;

// fn function(x: Rc<String>) {
//     println!("{}", x);
// }

// fn main() {
//     let a = Rc::new("Hello".to_string());
//     function(a.clone());
//     println!("{}", a);
// }

// use std::rc::Rc;

// struct Person {
//     name: Rc<String>,
// }

// impl Person {
//     fn name(x: Rc<String>) -> Self {
//         Person { name: x }
//     }
// }

// fn main() {
//     let a = Rc::new("Hello".to_string());

//     println!("No of smart pointers: {}", Rc::strong_count(&a));

//     let b = Person::name(a.clone());
//     println!("No of smart pointers: {}", Rc::strong_count(&a));

//     {
//         let c: Person = Person::name(a.clone());
//         println!("No of smart pointers: {}", Rc::strong_count(&a));
//     }

//     println!("No of smart pointers: {}", Rc::strong_count(&a));
//     println!("{}", a);
// }

// use std::rc::Rc;
// use std::cell::RefCell;

// fn main() {
//     let value = Rc::new(RefCell::new(5));
//     let b = Rc::new(Rc::clone(&value));

//     *b.borrow_mut() += 10;

//     println!("{:?}", b);
// }

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 0..10 {
//             println!("New thread {}", i);
//             thread::sleep(Duration::from_secs(2));
//         }
//     });

//     handle.join();

//     for i in 0..5 {
//         println!("Main thread {}", i);
//         thread::sleep(Duration::from_secs(2));
//     }
// }

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("vector value is: {:?}", v);
//     });

//     handle.join();
// }

// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         // let value = String::from("Hi");
//         // tx.send(value).unwrap();
//         let value = vec![
//             String::from("Hi"),
//             String::from("welcome"),
//             String::from("to"),
//             String::from("Rust")
//         ];

//         for i in value {
//             tx.send(i).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     // let received = rx.recv().unwrap();
//     // println!("Got the message: {}", received.as_str());

//     for received in rx {
//         println!("{}", received);
//     }
// }

// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Add for Point<T> where T: Add<Output = T> {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         Self { x: self.x + other.x, y: self.y + other.y }
//     }
// }

// fn main() {
//     let coord = Point { x: 5.0, y: 5.0 };
//     let coord2 = Point { x: 1.0, y: 2.0 };
//     let coord3 = coord.add(coord2);
//     println!("{:?}", coord3);
// }

// #[derive(Debug)]
// struct Car {
//     mpg: i8,
//     color: String,
//     top_speed: i16,
// }

// #[derive(Debug)]
// struct Motorcycle {
//     mpg: i8,
//     color: String,
//     top_speed: i16,
// }

// pub trait Properties {
//     fn set_mpg(&mut self, mpg: i8);
//     fn set_color(&mut self, color: String);
//     fn set_top_speed(&mut self, top_speed: i16);
// }

// impl Properties for Car {
//     fn set_mpg(&mut self, mpg: i8) {
//         self.mpg = mpg;
//     }

//     fn set_color(&mut self, color: String) {
//         self.color = color;
//     }

//     fn set_top_speed(&mut self, top_speed: i16) {
//         self.top_speed = top_speed;
//     }
// }

// impl Properties for Motorcycle {
//     fn set_mpg(&mut self, mpg: i8) {
//         self.mpg = mpg;
//     }

//     fn set_color(&mut self, color: String) {
//         self.color = color;
//     }

//     fn set_top_speed(&mut self, top_speed: i16) {
//         self.top_speed = top_speed;
//     }
// }

// fn print<T: std::fmt::Debug>(value: T) {
//     println!("{:?}", value);
// }

// fn main() {
//     let mut car = Car {
//         mpg: 0,
//         color: String::from("Red"),
//         top_speed: 0,
//     };

//     let mut motorcycle = Motorcycle {
//         mpg: 0,
//         color: String::from("Green"),
//         top_speed: 0,
//     };

//     car.set_mpg(15);
//     car.set_color(String::from("blue"));
//     car.set_top_speed(250);

//     motorcycle.set_mpg(100);
//     motorcycle.set_color(String::from("red"));
//     motorcycle.set_top_speed(2500);

//     print(car);
//     print(motorcycle);
// }

// use std::fs::File;

// fn main() {
//     // panic!("This will cause the program to abruptly end");

//     // let f = File::open("doesnotexist.txt").expect("No such thing");
//     let f = File::open("doesnotexist.txt").unwrap();

//     println!("The program is working here");
// }

// fn main() {
//     let solution = is_seven(7).unwrap();
//     print!("{}", solution);

//     let solution = is_seven(6).unwrap();
//     print!("{}", solution);
// }

// fn is_seven(n: i32) -> Result<bool, String> {
//     if n == 7 { Ok(true) } else { Err("This is not seven".to_string()) }
// }

// mod error;

// extern crate mylib;
// use crate::error::*;
// use crate::mylib::*;

// fn main() -> Result<(), TransactionError> {
//     let trans = get_transactions_b("test_data/transactions.json").expect(
//         "Could not load transactions"
//     );
//     for t in trans {
//         println!("{:?}", t);
//     }

//     // First trans
//     let t = get_first_transaction_for("test_data/transactions.json", "abc").ok_or(
//         "could not get first transaction"
//     )?;

//     println!("The first transaction is: {:?}", t);

//     Ok(())
// }

/*
fn main() {
    /*
    let l: [u8; 5] = [5, 6, 7, 8, 9];
    println!("{:?}", l.as_ptr());

    unsafe {
        let temp = std::ptr::read(l.as_ptr() as *const u8);
        println!("{}", temp);

        let temp = std::ptr::read(((l.as_ptr() as isize) + (1 as isize)) as *const u8);
        println!("{}", temp);

        let temp = std::ptr::read(((l.as_ptr() as isize) + (2 as isize)) as *const u8);
        println!("{}", temp);

        let temp = std::ptr::read(((l.as_ptr() as isize) + (3 as isize)) as *const u8);
        println!("{}", temp);

        let temp = std::ptr::read(((l.as_ptr() as isize) + (4 as isize)) as *const u8);
        println!("{}", temp);

        let temp = std::ptr::read(((l.as_ptr() as isize) + (5 as isize)) as *const u8);
        println!("{}", temp);

        let temp = std::ptr::read(
            ((l.as_ptr() as isize) + ((std::mem::size_of::<i32>() * 3) as isize)) as *const u8
        );
        println!("{}", temp);
    }
    */

    /*
    let l: [i32; 5] = [5, 6, 7, 8, 9];

    for i in l.iter() {
        println!("{}", i);
    }

    for i in 0..l.len() {
        // println!("{}", l[i]);

        let res = l.get(i);
        match res {
            Some(value) => println!("{}", value),
            None => println!("None"),
        }
    }
    */

    /*
    let mut l: [i32; 5] = [5, 6, 7, 8, 9];
    /*
    for i in l.iter_mut() {
        *i *= 2;
        println!("{}", i);
    }
    */

    for i in l.into_iter() {
        println!("{}", i);
    }
    */

    /*
    let mut l: [i32; 5] = [5, 6, 7, 8, 9];
    // println!("{}", l.contains(&8));

    let t = l
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
    println!("{:?}", t);
    */

    let mut l: [i32; 100] = [0; 100];

    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    v1.push(100);

    let v2: Vec<i32> = Vec::new();

    let temp = v1.pop().unwrap();
    println!("{}", temp);

    let t: i32 = v1.iter().sum();
    println!("{}", t);

    let t = v1
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
    println!("{:?}", t);

    let t = (0..100).collect::<Vec<i32>>();
    println!("{:?}", t);

    let mut v3: Vec<Vec<i32>> = Vec::new();
    v3.push(v1);
    println!("{:?}", v3);

    let mut name: Vec<char> = vec!['T', 'e', 't'];
    name.push('a');
    println!("{:?}", name);

    let name: &str = "laptop";
    for i in name.chars() {
        println!("{}", i);
    }

    let mut myname = name.to_string();
    println!("{}", myname);

    myname.push_str(" is heating");
    println!("{}", myname);

    let u: Vec<u8> = vec![1, 2, 3, 4, 5];
    println!("{:?}", String::from_utf8_lossy(&u[..]));
}
*/
