pub mod header;
use super::ParseError;

use crate::ast::Document;

pub trait DocParser {
    fn parse_document(&mut self) -> Result<Document, ParseError>;
}

pub mod body;
