#[derive(Debug)]
pub struct StringHolder<'a> {
    s: &'a str,
    t: &'a str,
}

fn main() {
    let mut s = make_str(7);
    // s.push_str(" people");

    s = to_people(s);
    to_frogs(&mut s);

    let p = part(&s);

    // not allowed to modify `s` because it is borrowed
    // s.push_str("people");

    println!("p = {}", p.to_string());
    println!("s = {}", s.to_string());

    let tog = two_strings(p, &s);

    println!("Tog = {:?}", tog);
    s.push_str(" anything");
    println!("final string = {}", s.to_string());
}

fn to_people(mut s: String) -> String {
    s.push_str(" people");
    s
}

fn to_frogs(s: &mut String) {
    s.push_str(" frogs")
}

fn make_str(n: i32) -> String {
    format!("hello {}", n)
}

fn part<'a>(s: &'a str) -> &'a str {
    if s.len() > 4 { &s[0..4] } else { s }
}

pub fn two_strings<'a>(s: &'a str, t: &'a str) -> StringHolder<'a> {
    StringHolder { s, t }
}
