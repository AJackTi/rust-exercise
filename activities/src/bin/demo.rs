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

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages:5,
        rating: 9,
    };

    display_page_count(&book);
    display_rating(&book);
}