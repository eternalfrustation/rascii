use crate::{ast::DocumentContent, checkpoint_iterator::CheckpointIterator};

use super::traits::body::DocContentParser;

impl<T> DocContentParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_document_content(&mut self) -> Result<DocumentContent, super::ParseError> {
        todo!()
    }
}
