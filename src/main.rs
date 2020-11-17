use assembler_rust::*;
use std::io;

fn main() {
    println!("Select:");
    println!("1. Convert string input to int");
    println!("2. Find and output pythagorean numbers");
    println!("3. Exit");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let selection = buffer.chars().nth(0).unwrap().to_digit(10).unwrap_or(9);

    match selection {
        1 => parser::run(),
        2 => pythagorean_numbers::run(),
        3 => return,
        _ => main(),
    };

    main();
}
