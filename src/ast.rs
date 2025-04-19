use chrono::NaiveDate;
use http::Uri;

#[derive(Debug, Clone)]
pub struct Document {
    pub header: Option<DocumentHeader>,
    pub content: DocumentContent,
}

#[derive(Debug, Clone)]
pub struct DocumentHeader {
    pub title: Option<String>,
    pub authors: Vec<Author>,
    pub revision: Revision,
}

#[derive(Debug, Clone)]
pub struct Author {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<Uri>,
}

#[derive(Debug, Clone)]
pub struct Revision {
    pub version: Vec<isize>,
    pub date: Option<NaiveDate>,
    pub remark: String,
}

#[derive(Debug, Clone)]
pub struct DocumentContent {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub title: String,
    pub attributes: Vec<Attribute>,
    pub content: BlockContent,
}

// TODO: Make this an enum
#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum BlockContent {
    List(Vec<ListContent>),
    Section(Vec<SectionContent>),
    Delimited(Vec<DelimitedBlockContent>),
    Undelimited(Vec<UndelimitedBlockContent>),
}

#[derive(Debug, Clone)]
pub enum ListContent {
    UnorderedList(UnorderedListContent),
    OrderedList(OrderedListContent),
}

#[derive(Debug, Clone)]
pub struct UnorderedListContent {
    pub text: String,
    pub sublist: Vec<ListContent>,
}

#[derive(Debug, Clone)]
pub struct OrderedListContent {
    pub text: String,
    pub sublist: Vec<ListContent>,
}

#[derive(Debug, Clone)]
pub enum SectionContent {
    Text(String),
    Block(Block),
}

#[derive(Debug, Clone)]
pub enum DelimitedBlockContent {
    Text(String),
    Block(Block),
}

#[derive(Debug, Clone)]
pub enum UndelimitedBlockContent {
    Text(String),
    Block(Block),
}
