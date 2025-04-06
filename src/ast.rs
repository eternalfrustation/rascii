use chrono::NaiveDate;
use http::Uri;

pub struct Document<'a> {
    pub header: Option<DocumentHeader<'a>>,
    pub content: DocumentContent<'a>,
}

pub struct DocumentHeader<'a> {
    pub title: Option<&'a str>,
    pub authors: Vec<Author<'a>>,
    pub revision: Revision<'a>,
}

pub struct Author<'a> {
    pub first_name: &'a str,
    pub middle_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub email: Option<Uri>,
}

pub struct Revision<'a> {
    pub version: Vec<&'a str>,
    pub date: Option<NaiveDate>,
    pub remark: &'a str,
}

pub struct DocumentContent<'a> {
    pub blocks: Vec<Block<'a>>,
}

pub struct Block<'a> {
    pub title: &'a str,
    pub attributes: Vec<Attributes>,
    pub content: BlockContent<'a>,
}

pub enum Attributes {}

pub enum BlockContent<'a> {
    List(Vec<ListContent<'a>>),
    Section(Vec<SectionContent<'a>>),
    Delimited(Vec<DelimitedBlockContent<'a>>),
    Undelimited(Vec<UndelimitedBlockContent<'a>>),
}

enum ListContent<'a> {
    UnorderedList {
        text: &'a str,
        sublist: Vec<ListContent<'a>>,
    },
    OrderedList {
        text: &'a str,
        sublist: Vec<ListContent<'a>>,
    },
}

enum SectionContent<'a> {
    Text(&'a str),
    Block(Block<'a>),
}

enum DelimitedBlockContent<'a> {
    Text(&'a str),
    Block(Block<'a>),
}

enum UndelimitedBlockContent<'a> {
    Text(&'a str),
    Block(Block<'a>),
}
