use std::fs;

pub mod ast;
pub mod parser;

fn main() {
    let contents = fs::read_to_string("test.adoc").expect("Something went wrong reading the file");
}
