use pest::{Parser, iterators::Pair};
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

use lexer::Rule;

pub mod lexer;

fn main() {
    let contents = fs::read_to_string("test.adoc").expect("Something went wrong reading the file");
    let lexer = lexer::AsciiDocParser::parse(Rule::document, &contents)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    print_pair(lexer, 5);
}

fn print_pair<R>(pair: Pair<R>, level: u8)
where
    R: Copy,
    R: Debug,
    R: Eq,
    R: Hash,
    R: Ord,
{
    if level < 1 {
        return;
    }
    println!("Rule:    {:?}", pair.as_rule());
    println!("Span:    {:?}", pair.as_span());
    println!("Text:    {}", pair.as_str());
    for inner_pair in pair.into_inner() {
        print_pair(inner_pair, level - 1);
    }
}
