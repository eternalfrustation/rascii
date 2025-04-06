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

fn parse_document_header<'a>(input: &'a str) -> IResult<&'a str, DocumentHeader<'a>> {
    let (input, heading) = opt(preceded((many0(newline), tag("= ")), parse_line)).parse(input)?;
    let (input, authors) = parse_authors(input)?;
    let (input, revision) = parse_revision_line(input)?;
    Ok((
        input,
        DocumentHeader {
            title: heading,
            authors,
            revision,
        },
    ))
}

fn parse_revision_line<'a>(input: &'a str) -> IResult<&'a str, Revision<'a>> {
    let (input, version) = parse_revision_number(input)?;
    let (input, date) = preceded((space0, tag(","), space0), parse_date).parse(input)?;
    let (input, remark) = preceded(tag(": "), is_not("\r\n")).parse(input)?;
    Ok((
        input,
        Revision {
            version,
            date,
            remark,
        },
    ))
}

fn parse_date<'a>(input: &'a str) -> IResult<&'a str, Option<chrono::NaiveDate>> {
    let (input, (year, _, month, _, day)) = (
        map_res(take(4u8), |i| i32::from_str_radix(i, 10)),
        tag("-"),
        map_res(take(2u8), |i| u32::from_str_radix(i, 10)),
        tag("-"),
        map_res(take(2u8), |i| u32::from_str_radix(i, 10)),
    )
        .parse(input)?;

    Ok((input, chrono::NaiveDate::from_ymd_opt(year, month, day)))
}

fn parse_revision_number<'a>(input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    preceded(opt(tag("v")), parse_version).parse(input)
}
fn parse_version<'a>(input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    separated_list1(tag("."), decimal).parse(input)
}
fn decimal<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    recognize(many1(terminated(alphanumeric1, many0(tag("_"))))).parse(input)
}

fn parse_authors<'a>(input: &'a str) -> IResult<&'a str, Vec<Author<'a>>> {
    separated_list1(tag(";"), parse_author).parse(input)
}

fn parse_author<'a>(input: &'a str) -> IResult<&'a str, Author<'a>> {
    let (input, (first_name, middle_name, last_name, email)) = (
        alphanumeric1,
        opt(preceded(tag(" "), alphanumeric1)),
        opt(preceded(tag(" "), alphanumeric1)),
        opt(preceded(tag(" "), delimited(tag("<"), parse_url, tag(">")))),
    )
        .parse(input)?;

    Ok((
        input,
        Author {
            first_name,
            middle_name,
            last_name,
            email,
        },
    ))
}

fn parse_url<'a>(input: &'a str) -> IResult<&'a str, Uri> {
    map_res(
        alphanumeric1.or(is_a("-._~:/?#[]@!$&'()*+,;=%")),
        |i: &str| i.parse(),
    )
    .parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    is_not("\r\n").parse(input)
}

fn parse_document_content<'a>(input: &'a str) -> IResult<&'a str, DocumentContent<'a>> {
    let (input, blocks) = separated_list0(space0, parse_block).parse(input)?;
    Ok((input, DocumentContent { blocks }))
}

fn parse_block<'a>(input: &'a str) -> IResult<&'a str, Block<'a>> {
    let (input, title) = parse_block_title(input)?;

    let (input, attributes) = parse_attribute_list(input)?;

    let (input, content) = parse_block_content(input)?;

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

fn parse_block_content<'a>(input: &'a str) -> IResult<&'a str, BlockContent<'a>> {
    alt((
        map(parse_list_content, BlockContent::List),
        map(parse_section_content, BlockContent::Section),
        map(parse_delimited_block_content, BlockContent::Delimited),
        map(parse_undelimited_block_content, BlockContent::Undelimited),
    ))
    .parse(input)
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
    todo!()
}

fn parse_undelimited_block_content<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<UndelimitedBlockContent<'a>>> {
    todo!()
}
