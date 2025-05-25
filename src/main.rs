use std::{fs, io::Read};

use checkpoint_iterator::CheckpointIterator;
use parser::DocHeaderParser;

pub mod ast;
pub mod checkpoint_iterator;
pub mod parser;

fn main() {
    pretty_env_logger::init();
    let mut test_file = fs::File::open("test.adoc").expect("The test file to be present");
    let mut buf = String::with_capacity(2048);
    test_file
        .read_to_string(&mut buf)
        .expect("To be able to read the test file");
    println!(
        "{:#?}",
        CheckpointIterator::new(buf.chars()).parse_document_header()
    );
}
