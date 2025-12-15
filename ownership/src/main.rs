use std::mem;

fn swap(first: &mut String, last: &mut String) {
    mem::swap(first, last);
}
fn greet(first: &String, last: &String) {
    println!("{first} {last}");
}

fn generate_name() -> (String, String) {
    let first = String::from("Joseph");
    let last = String::from("Tadesse");

    (first, last)
}

fn main() {
    let (mut first, mut last) = generate_name();
    greet(&first, &last);
    println!("{first} {last}");
    swap(&mut first, &mut last);
    println!("{first} {last}");

    let name = String::from("Joseph Tadesse");
    let last = &name[7..];
    println!("{last}");
}
