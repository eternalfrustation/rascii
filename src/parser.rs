use traits::{DocParser, body::DocContentParser, header::DocHeaderParser};

use crate::{ast::Document, checkpoint_iterator::CheckpointIterator};

pub mod header;
pub mod body;
pub mod traits;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub start: usize,
    pub end: usize,
    pub message: String,
}

impl<T> DocParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_document(&mut self) -> Result<Document, ParseError> {
        Ok(Document {
            header: self.opt_parse(Self::parse_document_header),
            content: self.parse_document_content()?,
        })
    }
}
