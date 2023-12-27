use std::io;

pub fn inputs(){
    let mut input = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Hello, {}", input);
}