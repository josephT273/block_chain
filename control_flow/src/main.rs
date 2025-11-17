fn main() {
    let condition = true;
    let number = if condition {3} else {5};

    let x;
    if condition {
        x = 12;
    }else{
        x = 10;
    }
    println!("{x}");
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}