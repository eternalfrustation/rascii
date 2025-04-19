use http::Uri;

use crate::{
    ast::{Author, DocumentHeader, Revision},
    parser::ParseError,
    checkpoint_iterator::CheckpointIterator,
};

use super::{
    AuthorParser, AuthorsParser, DateParser, DecimalParser, DocHeaderParser, LineParser,
    RevisionLineParser, UriParser, VersionParser,
};

impl<T> DocHeaderParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_document_header(&mut self) -> Result<DocumentHeader, super::ParseError> {
        let _ = self.take_while_ref(|c| c.is_whitespace() || *c == '=');
        let title = Some(
            self.parse_line()
                .expect("Parsing line for header should never return an error"),
        );
        let authors = self.parse_authors()?;
        let revision = self.parse_revision_line()?;
        Ok(DocumentHeader {
            title,
            authors,
            revision,
        })
    }
}

impl<T> RevisionLineParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_revision_line(&mut self) -> Result<Revision, super::ParseError> {
        let version = match self.parse_version() {
            Err(_) => vec![],
            Ok(v) => v,
        };

        let _ = self.take_while_ref(|c| c.is_whitespace() || *c == ',');
        let date = self.parse_date().ok();
        let _ = self.take_while_ref(|c| c.is_whitespace() || *c == ':');
        let remark = self.parse_line().expect("Parsing line shouldn't ever fail");
        Ok(Revision {
            version,
            date,
            remark,
        })
    }
}

impl<T> DateParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_date(&mut self) -> Result<chrono::NaiveDate, super::ParseError> {
        let year = self
            .take_while_ref(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .map_err(|e| self.error(format!("Error while parsing year: {e}")))?;
        let month = self
            .take_while_ref(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .map_err(|e| self.error(format!("Error while parsing year: {e}")))?;
        let day = self
            .take_while_ref(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .map_err(|e| self.error(format!("Error while parsing year: {e}")))?;
        match chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Some(v) => Ok(v),
            None => Err(self.error(String::from("Invalid date provided"))),
        }
    }
}

impl<T> VersionParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_version(&mut self) -> Result<Vec<isize>, super::ParseError> {
        let mut version = Vec::new();
        while let Ok(v) = self.parse_decimal() {
            version.push(v);
        }
        Ok(version)
    }
}

impl<T> DecimalParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_decimal(&mut self) -> Result<isize, super::ParseError> {
        self.take_while_ref(|e| e.is_numeric() || *e == '_')
            .filter(|e| e.is_numeric())
            .collect::<String>()
            .parse()
            .map_err(|e| self.error(format!("Error while parsing int {e}")))
    }
}

impl<T> AuthorsParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_authors(&mut self) -> Result<Vec<Author>, super::ParseError> {
        let mut authors = Vec::new();
        while let Ok(author) = self.parse_author() {
            authors.push(author)
        }
        Ok(authors)
    }
}

impl<T> AuthorParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_author(&mut self) -> Result<Author, super::ParseError> {
        let first_name = String::from(
            self.take_while_ref(|i| !i.is_whitespace())
                .collect::<String>()
                .trim(),
        );

        if first_name.len() == 0 {
            return Err(self.error("No Content in Author Line".to_string()));
        }

        let middle_name = match self.parse_url() {
            Err(_) => Some(self.take_while_ref(|i| !i.is_whitespace()).collect::<String>()),
            Ok(v) => {
                return Ok(Author {
                    first_name,
                    middle_name: None,
                    last_name: None,
                    email: Some(v),
                });
            }
        };
        let last_name = match self.parse_url() {
            Err(_) => Some(self.take_while_ref(|i| !i.is_whitespace()).collect::<String>()),
            Ok(v) => {
                return Ok(Author {
                    first_name,
                    middle_name: None,
                    last_name: middle_name,
                    email: Some(v),
                });
            }
        };
        let email = if let Some('<') = self.next() {
            let email = Some(self.parse_url()?);
            if let Some('>') = self.next() {
            } else {
                return Err(self.error("Matching > not found for <".to_string()));
            }
            email
        } else {
            None
        };
        Ok(Author {
            first_name,
            middle_name,
            last_name,
            email,
        })
    }
}

impl<T> UriParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_url(&mut self) -> Result<Uri, super::ParseError> {
        let start_pos = self.push();
        match self
            .take_while_ref(|i| !i.is_alphanumeric() || "-._~:/?#[]@!$&'()*+,;%=".contains(*i))
            .collect::<String>()
            .parse()
        {
            Err(e) => {
                let end_pos = self
                    .pop()
                    .expect("Push executed before pop on checkpoint iterator");
                log::warn!("URI Parsing failed with error: {e}");
                return Err(ParseError {
                    start: start_pos,
                    end: end_pos,
                    message: format!("URI Parsing Failed with error: {e}"),
                });
            }
            Ok(v) => Ok(v),
        }
    }
}

impl<T> LineParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_line(&mut self) -> Result<String, super::ParseError> {
        Ok(self
            .take_while_ref(|i| !i.is_ascii_control())
            .collect::<String>())
    }
}
