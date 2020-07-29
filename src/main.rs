mod lexer;
use crate::lexer::*;
mod parser;
use crate::parser::*;
fn main() {
    println!("Hello, world!");
   println!("{:?}", tokenize(&"def(),;[]pole(hi)34externdef"));
}
