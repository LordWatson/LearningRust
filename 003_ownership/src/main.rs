fn main() {
    demonstrate_ownership();
    demonstrate_borrowing();
    demonstrate_mutable_borrowing();
}

fn demonstrate_ownership() {
    let s = String::from("Hello, Ownership!");
    // Passing ownership
    {
        take_ownership(s);
    }

    // Using `s` here would cause a compile error because ownership was moved.
    // Uncommenting the next line will result in a compile-time error:
    // println!("{}", s);
}

fn take_ownership(some_string: String) {
    println!("I took ownership of: {}", some_string);
    // `some_string` is dropped after this function's scope ends.
}

fn demonstrate_borrowing() {
    let s = String::from("Hello, Borrowing!");
    let length = calculate_length(&s); // Borrowing `s` without transferring ownership
    println!("Length of '{}' is {}", s, length); // Works because ownership wasn't moved
}

fn calculate_length(s: &String) -> usize {
    s.len() // Can only use `s` for reading; no modifications allowed
}

fn demonstrate_mutable_borrowing() {
    let mut s = String::from("Hello");
    modify(&mut s); // Borrowing `s` mutably
    println!("Modified string: {}", s);
}

fn modify(some_string: &mut String) {
    some_string.push_str(", mutable borrowing!");
}