use std::{fs, io::Read};

use parser::DocHeaderParser;
use checkpoint_iterator::CheckpointIterator;

pub mod ast;
pub mod parser;
pub mod checkpoint_iterator;

fn main() {
    let mut test_file = fs::File::open("test.adoc").expect("The test file to be present");
    let mut buf = String::with_capacity(2048);
    test_file.read_to_string(&mut buf).expect("To be able to read the test file");
    println!("{:?}", CheckpointIterator::new(buf.chars()).parse_document_header());
}
