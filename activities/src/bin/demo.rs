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

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name:"cereal".to_owned(),
            count: 1
        }, 
        LineItem {
            name: String::from("fruit"),
            count: 3
        }
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}