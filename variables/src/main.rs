const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn shadowing(){
 let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x ix {x}");
    }


    println!("The Value of x is {x}");

}

fn constant() {
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

fn main() {
    println!("Constants");
    constant();

    println!("Shadowing");
    shadowing();

}
