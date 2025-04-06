pub mod in_memory;
use crate::ast::Document;
use nom::IResult;

pub trait AsciiDocParser<'a> {
    type Input;
    fn try_to_ast(&self, input: Self::Input) -> IResult<Self::Input, Document<'a>>;
}
