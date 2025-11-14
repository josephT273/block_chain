fn char_type() {
    let a = 'A';
    let z: char = 'Z';
    println!("{a} {z}");
}

fn bool_type() {
    let is_exist = true;
    let is_valid: bool = false;
    println!("Is Exist {is_exist} and is valid {is_valid}");
}



fn main() {
    char_type();
    bool_type();

    // addition
    let sum = 5 + 10;
    println!("Sum of 5 + 10 = {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference of 95.5 - 4.3 = {difference}");
    // multiplication
    let product = 4 * 30;
    println!("Multiplication of 4 * 30 = {product}");
    // division
    let quotient = 56.7 / 32.2;
    println!("Division of 56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("Trancuated devision of -5 / 3 = {truncated}");
    // remainder
    let remainder = 43 % 5;

    println!("Remainder of 43 / 5 = {remainder}");
}
