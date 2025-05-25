pub mod header;
use http::Uri;

use crate::ast::{Author, Document, DocumentHeader, Revision, SectionHeading};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub start: usize,
    pub end: usize,
    pub message: String,
}

pub trait DocParser {
    fn parse_document(&mut self) -> Result<Document, ParseError>;
}

pub trait DocHeaderParser {
    fn parse_document_header(&mut self) -> Result<DocumentHeader, ParseError>;
}

pub trait DocSectionHeading {
    fn parse_section_heading(&mut self) -> Result<SectionHeading, ParseError>;
}


pub trait RevisionLineParser {
    fn parse_revision_line(&mut self) -> Result<Revision, ParseError>;
}

pub trait DateParser {
    fn parse_date(&mut self) -> Result<chrono::NaiveDate, ParseError>;
}

pub trait VersionParser {
    fn parse_version(&mut self) -> Result<Vec<isize>, ParseError>;
}

pub trait DecimalParser {
    fn parse_decimal(&mut self) -> Result<isize, ParseError>;
}

pub trait AuthorsParser {
    fn parse_authors(&mut self) -> Result<Vec<Author>, ParseError>;
}

pub trait AuthorParser {
    fn parse_author(&mut self) -> Result<Author, ParseError>;
}

pub trait UriParser {
    fn parse_url(&mut self) -> Result<Uri, ParseError>;
}

pub trait LineParser {
    fn parse_line(&mut self) -> Result<String, ParseError>;
}
