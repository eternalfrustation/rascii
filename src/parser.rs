use crate::ast::{Author, Document, DocumentContent, DocumentHeader, Revision};
use http::Uri;
use nom::{
    IResult, Parser,
    bytes::{is_a, is_not, tag, take, take_while},
    character::{
        complete::{alphanumeric1, newline, not_line_ending, space0},
        one_of,
    },
    combinator::{map_res, not, opt, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated},
};

pub trait AsciiDocParser<'a> {
    type Input;
    fn try_to_ast(&self, input: Self::Input) -> IResult<Self::Input, Document<'a>>;
}

pub struct InMemoryParser {}

impl<'a> InMemoryParser {
    fn parse_document(&self, input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, header) = opt(|i| self.parse_document_header(i)).parse(input)?;
        let (input, content) = self.parse_document_content(input)?;
        Ok((input, Document { header, content }))
    }

    fn parse_document_header(&self, input: &'a str) -> IResult<&'a str, DocumentHeader<'a>> {
        let (input, heading) = opt(preceded((many0(newline), tag("= ")), |i| {
            self.parse_line(i)
        }))
        .parse(input)?;
        let (input, authors) = self.parse_authors(input)?;
        let (input, revision) = self.parse_revision_line(input)?;
        Ok((
            input,
            DocumentHeader {
                title: heading,
                authors,
                revision,
            },
        ))
    }

    fn parse_revision_line(&self, input: &'a str) -> IResult<&'a str, Revision<'a>> {
        let (input, revision_number) = self.parse_revision_number(input)?;
        let (input, date) =
            preceded((space0, tag(","), space0), |i| self.parse_date(i)).parse(input)?;
        let (input, remark) = preceded(tag(": "), is_not("\r\n")).parse(input)?;
        Ok((
            input,
            Revision {
                version: revision_number,
                date,
                remark,
            },
        ))
    }

    fn parse_date(&self, input: &'a str) -> IResult<&'a str, Option<chrono::NaiveDate>> {
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

    fn parse_revision_number(&self, input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        preceded(opt(tag("v")), |i| self.parse_version(i)).parse(input)
    }
    fn parse_version(&self, input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        separated_list1(tag("."), |i| self.decimal(i)).parse(input)
    }
    fn decimal(&self, input: &'a str) -> IResult<&'a str, &'a str> {
        recognize(many1(terminated(alphanumeric1, many0(tag("_"))))).parse(input)
    }

    fn parse_authors(&self, input: &'a str) -> IResult<&'a str, Vec<Author<'a>>> {
        separated_list1(tag(";"), |i| self.parse_author(i)).parse(input)
    }

    fn parse_author(&self, input: &'a str) -> IResult<&'a str, Author<'a>> {
        let (input, (first_name, middle_name, last_name, email)) = (
            alphanumeric1,
            opt(preceded(tag(" "), alphanumeric1)),
            opt(preceded(tag(" "), alphanumeric1)),
            opt(preceded(
                tag(" "),
                delimited(tag("<"), |i| self.parse_url(i), tag(">")),
            )),
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

    fn parse_url(&self, input: &'a str) -> IResult<&'a str, Uri> {
        map_res(
            alphanumeric1.or(is_a("-._~:/?#[]@!$&'()*+,;=%")),
            |i: &str| i.parse(),
        )
        .parse(input)
    }

    fn parse_line(&self, input: &'a str) -> IResult<&'a str, &'a str> {
        is_not("\r\n").parse(input)
    }

    fn parse_document_content(&self, input: &'a str) -> IResult<&'a str, DocumentContent<'a>> {
        todo!()
    }
}

impl<'a> AsciiDocParser<'a> for InMemoryParser {
    type Input = &'a str;
    fn try_to_ast(&self, input: &'a str) -> IResult<Self::Input, Document<'a>> {
        self.parse_document(input)
    }
}
