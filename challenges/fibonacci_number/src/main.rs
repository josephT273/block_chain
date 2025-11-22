fn fibonacci_number(n: i32) -> i32{
    if n == 1 {
        return n;
    }
    return fibonacci_number(n - 1) + n;
}

fn main() {
    let x: i32 = fibonacci_number(6);
    println!("Fibonacci number for 6 is: {x}");
}
