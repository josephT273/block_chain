use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");

    let mut y: i32 = match x.trim().parse::<i32>(){
        Ok(number ) => number,
        Err(_) => todo!(),
    };

    let temp = y * 9 / 5 + 32;
    println!("{}", temp);
}
