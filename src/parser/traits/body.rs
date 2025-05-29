use crate::ast::{
    Attribute, Block, BlockContent, DelimitedBlockContent, DocumentContent, ListContent,
    SectionContent, UndelimitedBlockContent,
};

use super::ParseError;

pub trait DocContentParser {
    fn parse_document_content(&mut self) -> Result<DocumentContent, ParseError>;
}

pub trait DocBlockParser {
    fn parse_block(&mut self) -> Result<Block, ParseError>;
    fn parse_section_block(&mut self) -> Result<Block, ParseError>;
    fn parse_list_block(&mut self) -> Result<Block, ParseError>;
    fn parse_delimited_block(&mut self) -> Result<Block, ParseError>;
    fn parse_undelimited_block(&mut self) -> Result<Block, ParseError>;
}

pub trait DocAttributeParser {
    fn parse_attribute(&mut self) -> Result<Attribute, ParseError>;
}

pub trait DocAttributesParser {
    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, ParseError>;
}

pub trait DocBlockContentParser {
    fn parse_block_content(&mut self) -> Result<BlockContent, ParseError>;
}

pub trait DocListContentParser {
    fn parse_list_content(&mut self) -> Result<ListContent, ParseError>;
}

pub trait DocSectionContentParser {
    fn parse_section_content(&mut self) -> Result<Vec<SectionContent>, ParseError>;
}

pub trait DocDelimitedBlockContentParser {
    fn parse_delimited_block_content(&mut self) -> Result<DelimitedBlockContent, ParseError>;
}

pub trait DocUndelimitedBlockContentParser {
    fn parse_undelimited_block_content(&mut self) -> Result<UndelimitedBlockContent, ParseError>;
}
