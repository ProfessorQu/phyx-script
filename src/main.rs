use std::fs;

mod interpreter;
use interpreter::tokenize;

fn main() {
    let code = fs::read_to_string("ball.phyx").expect("Failed to read file");
    println!("tokens: {:?}", tokenize(code));
}
