use crate::ast::{
    Attribute, Author, Block, BlockContent, DelimitedBlockContent, Document, DocumentContent,
    DocumentHeader, ListContent, Revision, SectionContent, UndelimitedBlockContent,
};
use http::Uri;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{is_a, is_not, tag, take},
    character::complete::{alphanumeric1, newline, space0, space1},
    combinator::{map, map_res, not, opt, recognize},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
};

use super::AsciiDocParser;

pub struct InMemoryParser {}

impl<'a> AsciiDocParser<'a> for InMemoryParser {
    type Input = &'a str;
    fn try_to_ast(&self, input: &'a str) -> IResult<Self::Input, Document<'a>> {
        parse_document(input)
    }
}

fn parse_document<'a>(input: &'a str) -> IResult<&'a str, Document<'a>> {
    let (input, header) = opt(parse_document_header).parse(input)?;
    let (input, content) = parse_document_content(input)?;
    Ok((input, Document { header, content }))
}


fn parse_document_content<'a>(input: &'a str) -> IResult<&'a str, DocumentContent<'a>> {
    let (input, blocks) = separated_list0(space0, parse_block).parse(input)?;
    Ok((input, DocumentContent { blocks }))
}

fn parse_block_header<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Vec<Attribute<'a>>)> {
    let (input, title) = parse_block_title(input)?;

    let (input, attributes) = parse_attribute_list(input)?;

    Ok((input, (title, attributes)))
}

fn parse_block<'a>(input: &'a str) -> IResult<&'a str, Block<'a>> {
    let (input, (title, attributes)) = parse_block_header(input)?;
    let (input, content) = alt((
        map(parse_list_content, BlockContent::List),
        map(parse_section_content, BlockContent::Section),
        map(parse_delimited_block_content, BlockContent::Delimited),
        map(parse_undelimited_block_content, BlockContent::Undelimited),
    ))
    .parse(input)?;
    Ok((
        input,
        Block {
            title,
            attributes,
            content,
        },
    ))
}

fn parse_block_title<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    delimited((tag("."), not(space1)), is_not("\r\n"), newline).parse(input)
}

fn parse_attribute_list<'a>(input: &'a str) -> IResult<&'a str, Vec<Attribute<'a>>> {
    delimited(
        tag("["),
        separated_list1(tag(", "), parse_attribute),
        tag("]"),
    )
    .parse(input)
}

fn parse_attribute<'a>(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
    let (input, (key, value)) = separated_pair(
        alt((is_not("="), delimited(tag("\""), is_not("\""), tag("\"")))),
        tag("="),
        alt((is_not(","), delimited(tag("\""), is_not("\""), tag("\"")))),
    )
    .parse(input)?;
    Ok((input, Attribute { key, value }))
}

fn parse_list_content<'a>(input: &'a str) -> IResult<&'a str, Vec<ListContent<'a>>> {
    todo!()
}

fn parse_ordered_list<'a>(input: &'a str) -> IResult<&'a str, Vec<ListContent<'a>>> {
    todo!()
}

fn parse_section_content<'a>(input: &'a str) -> IResult<&'a str, Vec<SectionContent<'a>>> {
    todo!()
}

fn parse_delimited_block_content<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<DelimitedBlockContent<'a>>> {
    many1(alt((
        map(parse_delimited_subblocks, |block| {
            DelimitedBlockContent::Block(block)
        }),
        map(parse_line, |line| DelimitedBlockContent::Text(line)),
    )))
    .parse(input)
}

fn parse_delimited_subblocks<'a>(input: &'a str) -> IResult<&'a str, Block<'a>> {
    let (input, (title, attributes)) = parse_block_header(input)?;
    let (input, content) = alt((
        map(parse_list_content, BlockContent::List),
        map(parse_section_content, BlockContent::Section),
        map(parse_undelimited_block_content, BlockContent::Undelimited),
    ))
    .parse(input)?;
    Ok((
        input,
        Block {
            title,
            attributes,
            content,
        },
    ))
}


fn parse_undelimited_block_content<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<UndelimitedBlockContent<'a>>> {
    many1(alt((
        map(parse_undelimited_subblocks, |block| {
            UndelimitedBlockContent::Block(block)
        }),
        map(parse_line, |line| UndelimitedBlockContent::Text(line)),
    )))
    .parse(input)
}

fn parse_undelimited_subblocks<'a>(input: &'a str) -> IResult<&'a str, Block<'a>> {
    let (input, (title, attributes)) = parse_block_header(input)?;
    let (input, content) = alt((
        map(parse_list_content, BlockContent::List),
        map(parse_section_content, BlockContent::Section),
        map(parse_delimited_block_content, BlockContent::Delimited),
    ))
    .parse(input)?;
    Ok((
        input,
        Block {
            title,
            attributes,
            content,
        },
    ))
}
