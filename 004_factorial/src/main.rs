use std::io;

fn factorial(input_number: u64) -> u64 {
    if input_number <= 1 {
        return 1;
    }

    input_number * factorial(input_number - 1)
}

fn main() {
    println!("Enter a number: ");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");

    let num = input_text.trim()
        .parse::<u64>()
        .expect("That's not a number");

    println!("{}", factorial(num));
}