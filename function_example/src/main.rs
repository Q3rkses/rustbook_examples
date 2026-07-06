use std::io;

fn main() {
    println!("Hello, world!");

    let mut first_input = String::new();

    println!("Write down a number.");
    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read line");

    let first_input: i32 = first_input.trim().parse().expect("Not a valid number");

    another_function(first_input);
    print_labeled_measurement(5, 'h');

    let x = plus_one(5);
    println!("The value of x is: {x}");

    let number = first_input;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // If-let statements
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loop
    for number in (1..4).rev() {
        println!("{number}...");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) -> () {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
