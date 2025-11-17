fn main() {
    let y = {
        let x = 3;
        x + 2
    };

    println!("The value of Y: {y}");
    print_labeled_measurement(12, 'h');
    println!("The square of 5 is : {0}", another_function(5));
}

fn another_function(x: i32) -> i32 {
    x * x
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}")
}
