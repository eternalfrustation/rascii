use crate::{
    ast::{Attribute, Block, BlockContent, DocumentContent, SectionContent},
    checkpoint_iterator::CheckpointIterator,
    parser::traits::header::DocSectionHeading,
};

use super::traits::{
    body::{
        DocAttributeParser, DocAttributesParser, DocBlockParser, DocContentParser,
        DocSectionContentParser,
    },
    header::LineParser,
};

impl<T> DocContentParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_document_content(&mut self) -> Result<DocumentContent, super::ParseError> {
        let mut blocks = Vec::new();
        while let Ok(block) = self.parse_block() {
            blocks.push(block);
        }
        Ok(DocumentContent { blocks })
    }
}

impl<T> DocBlockParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_block(&mut self) -> Result<Block, super::ParseError> {
        self.parse_section_block()
            .or_else(|e| {
                log::error!("{e:?}");
                self.parse_list_block()
            })
            .or_else(|e| {
                log::error!("{e:?}");
                self.parse_delimited_block()
            })
            .or_else(|e| {
                log::error!("{e:?}");
                self.parse_undelimited_block()
            })
    }

    fn parse_section_block(&mut self) -> Result<Block, super::ParseError> {
        Ok(Block {
            heading: self.parse_section_heading()?,
            attributes: self.opt_parse(Self::parse_attributes).unwrap_or(Vec::new()),
            content: BlockContent::Section(self.parse_section_content()?),
        })
    }

    fn parse_list_block(&mut self) -> Result<Block, super::ParseError> {
        todo!("List parsing is not implemented")
    }

    fn parse_delimited_block(&mut self) -> Result<Block, super::ParseError> {
        todo!("Delimited block parsing is not implemented")
    }

    fn parse_undelimited_block(&mut self) -> Result<Block, super::ParseError> {
        todo!("Undelimited block parsing is not implemented")
    }
}

impl<T> DocAttributesParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, super::ParseError> {
        if let Some('[') = self.next() {
        } else {
            return Err(
                self.error("Expected '[' for the start of a list of attributes".to_string())
            );
        }
        let mut attributes = Vec::new();
        while let Ok(attribute) = self.parse_attribute() {
            attributes.push(attribute);
            if let Some(',') = self.next() {
            } else {
                return Err(self.error("Expected ',' after a attribute".to_string()));
            }
        }
        if let Some(']') = self.next() {
        } else {
            return Err(self.error("Expected ']' for the end of a list of attributes".to_string()));
        }
        Ok(attributes)
    }
}

impl<T> DocAttributeParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_attribute(&mut self) -> Result<Attribute, super::ParseError> {
        let key: String = self
            .take_while_ref(|c| *c != '=' && *c != ']' && *c != ',' && !c.is_ascii_control())
            .collect();
        if key.is_empty() {
            return Err(self.error("Empty key for attribute".to_string()));
        }
        let value: Option<String> = self.opt_parse(|s| {
            if let Some('=') = s.next() {
                Ok(
                    s.take_while_ref(|c| *c != ']' && *c != ',' && !c.is_ascii_control())
                        .collect(),
                )
            } else {
                Err(s.error("Expect '=' for specifying value of keys in attributes".to_string()))
            }
        });
        Ok(Attribute { key, value })
    }
}

impl<T> DocSectionContentParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_section_content(&mut self) -> Result<Vec<SectionContent>, super::ParseError> {
        let mut content = Vec::new();
        while let Ok(content_block) = self.parse_block().map(SectionContent::Block).or_else(|e| {
            log::error!("{e:?}");
            self.parse_line().map(SectionContent::Text)
        }) {
            content.push(content_block)
        }
        Ok(content)
    }
}
