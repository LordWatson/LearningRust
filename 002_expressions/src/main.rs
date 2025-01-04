use std::{io, process};

fn dd<T: std::fmt::Debug>(value: T) -> ! {
    // Print the value using debug formatting
    println!("{:#?}", value);
    // Exit the program with a successful exit code
    process::exit(0);
}

fn main() {
    let x:i32 = add(5, 10);

    println!("The value of x is: {}", x);
    println!();

    let hello:&str = "hello";

    let hello_world: String = format!("{} {}", hello, world());

    println!("The value of hello_world is: {}", hello_world);

    println!("The value of x is: {}", x);
    println!();

    // input 1
    let mut input1: String = String::new();
    println!("Input your first word");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    println!();

    let mut input2: String = String::new();
    println!("Input your second word");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    println!();

    let hello_world_str: String = join_strings(&input1, &input2);

    dd(hello_world_str);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn world() -> &'static str {
    "world"
}

fn join_strings(x: &str, y: &str) -> String {
    format!("{} {}", x, y)
}




