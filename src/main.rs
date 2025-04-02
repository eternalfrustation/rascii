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
    print_pair(lexer, 6, 6);
}

fn print_pair<R>(pair: Pair<R>, level: u8, max_level: u8)
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
    let indentation = String::from("  ").repeat((max_level - level) as usize);
    let rule = pair.as_rule();
    println!("{indentation}Rule:    {:?}", rule);
    let rule_str = format!("{:?}", rule);

    if rule_str.contains("line")
        || rule_str.contains("word")
        || rule_str.contains("number")
        || rule_str.contains("date")
        || rule_str.contains("delimited_block_characters")
    {
        println!("{indentation}Span:    {:?}", pair.as_span());
    }

    for inner_pair in pair.into_inner() {
        print_pair(inner_pair, level - 1, max_level);
    }
}
