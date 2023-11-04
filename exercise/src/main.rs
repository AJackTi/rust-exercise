// fn median(mut a: Vec<f32>) -> Option<f32> {
//     if a.is_empty() {
//         return None;
//     }

//     a.sort_by(|x, y| { x.partial_cmp(y).unwrap() });

//     let n_elements = a.len();
//     let middle = n_elements / 2;
//     let a_is_even = n_elements % 2 == 0;

//     let med = if a_is_even { (a[middle] + a[middle - 1]) / 2.0 } else { a[middle] };

//     Some(med)
// }

// fn unique(mut a: Vec<i32>) -> Vec<i32> {
// let mut tmp = Vec::new();

// for elem in a {
//     if tmp.contains(&elem) {
//         continue;
//     }
//     tmp.push(elem);
// }
// tmp
// }

// fn unique(mut a: Vec<i32>) -> Vec<i32> {
//     a.sort_unstable();
//     a.dedup();
//     a
// }

// fn unique<T: Ord>(mut a: Vec<T>) -> Vec<T> {
//     a.sort_by(|x: &T, y: &T| x.cmp(y));
//     a.dedup();
//     a
// }

// fn sort_username<T: AsRef<str>>(username: &mut Vec<T>) {
//     username.sort_by_cached_key(|x| { x.as_ref().to_lowercase() });

//     // username.sort_by(|x, y| x.as_ref().to_lowercase().cmp(&y.as_ref().to_lowercase()))
// }

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

// Represents a single character
type Letter = Vec<Pulse>;

// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;

        let mut msg = Vec::with_capacity(self.len());
        for c in self.chars() {
            let morse = match c {
                'A' | 'a' => vec![Short, Long],
                _ => {
                    continue;
                }
            };
            msg.push(morse);
        }

        msg
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    // let answer = median(vec![1.0, 2.0, 5.0, 6.0]);
    // dbg!(answer);

    // let answer = unique(vec![1, 1, 2, 3, 4, 4, 5]);
    // dbg!(answer);

    // let answer = unique(vec![1, 2, 3, 4, 4, 5, 5]);
    // dbg!(answer);

    // let mut users = vec!["Todd", "Amy", "Jennifer", "Tommy"];
    // sort_username(&mut users);
    // dbg!(users);

    let greeting = "aaa hello, world".to_string().to_morse_code();
    print_morse_code(&greeting)
}
