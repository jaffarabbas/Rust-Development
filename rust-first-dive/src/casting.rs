use std::io;

pub fn casting(){
    let mut input = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    //casting
    let input: u32 = input.trim().parse().unwrap();
    println!("Hello, {}", input);
}