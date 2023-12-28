mod helper;
mod owner_borrow;
mod scoping;
mod inputs;
mod casting;
mod conditions;
mod datatypes;
mod functions;

use owner_borrow::owner_borrow::{owner, borrow};
use scoping::scoping;
use datatypes::dataTypes;
use inputs::inputs;
use casting::casting;
use conditions::condition;
use functions::{add, sub};

pub fn test(){
    println!("Select your functions");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().unwrap();
    match input {
        1 => owner(),
        2 => borrow(),
        3 => scoping(),
        4 => dataTypes(),
        5 => inputs(),
        6 => casting(),
        7 => condition(),
        8 => println!("{}",add(1, 3)),
        9 => println!("{}",sub(1, 3)),
        _ => println!("Invalid input"),
    }
}

fn main() {
    test();
}
