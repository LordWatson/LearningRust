use std::process;

fn dd<T: std::fmt::Debug>(value: T) -> ! {
    // Print the value using debug formatting
    println!("{:#?}", value);
    // Exit the program with a successful exit code
    process::exit(0);
}

fn main() {
    let example_data = vec!["hello", "world", "rust"];
    dd(example_data);
}

