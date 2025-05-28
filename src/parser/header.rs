use http::Uri;

use crate::{
    ast::{Author, DocumentHeader, Revision, SectionHeading},
    checkpoint_iterator::CheckpointIterator,
    parser::ParseError,
};

use super::traits::header::{
    AuthorParser, AuthorsParser, DateParser, DecimalParser, DocHeaderParser, DocSectionHeading,
    LineParser, RevisionLineParser, UriParser, VersionParser,
};

impl<T> DocHeaderParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_document_header(&mut self) -> Result<DocumentHeader, super::ParseError> {
        let title = self.opt_parse(Self::parse_section_heading);
        let authors = self.opt_parse(Self::parse_authors).unwrap_or(Vec::new());
        let revision = self.opt_parse(Self::parse_revision_line);
        Ok(DocumentHeader {
            title,
            authors,
            revision,
        })
    }
}

impl<T> DocSectionHeading for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_section_heading(&mut self) -> Result<SectionHeading, super::ParseError> {
        // Skip all preceding whitespace from the start, count for consuming the iterator
        // Without count, iterator won't advance
        let whitespace_count = self.take_while_ref(|c| c.is_whitespace()).count();
        log::info!("Skipping {whitespace_count} Whitespaces before section heading");
        let level = self.take_while_ref(|c| *c == '=').count();
        log::info!("Section Level {level}");
        if level == 0 {
            return Err(self.error("Expected '='".to_string()));
        }
        let space_count = self.take_while_ref(|c| *c == ' ').count();
        if space_count == 0 {
            return Err(self.error("Expected ' '".to_string()));
        }
        let text = self.parse_line()?;
        Ok(SectionHeading { level, text })
    }
}

impl<T> RevisionLineParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_revision_line(&mut self) -> Result<Revision, super::ParseError> {
        let version = match self.parse_version() {
            Err(_) => {
                println!("Error while parsing version");
                vec![]
            }
            Ok(v) => v,
        };

        let date = self.opt_parse(|state| {
            let _ = state
                .take_while_ref(|c| c.is_whitespace() || *c == ',')
                .collect::<String>();
            state.parse_date()
        });
        let _ = self
            .take_while_ref(|c| c.is_whitespace() || *c == ':')
            .count();
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
            .take_while_ref(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .map_err(|e| self.error(format!("Error while parsing year: {e}")))?;
        self.take_while_ref(|v| *v == '-').count();
        let month = self
            .take_while_ref(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .map_err(|e| self.error(format!("Error while parsing month: {e}")))?;
        self.take_while_ref(|v| *v == '-').count();
        let day = self
            .take_while_ref(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .map_err(|e| self.error(format!("Error while parsing day: {e}")))?;
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
            self.take_while_ref(|v| *v == '.').next();
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
        let mut current_line = CheckpointIterator::new(self.take_while(|c| !c.is_ascii_control()));
        while let Ok(author) = current_line.parse_author() {
            authors.push(author.clone());
        }
        Ok(authors)
    }
}

impl<T> AuthorParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_author(&mut self) -> Result<Author, super::ParseError> {
        self.take_while_ref(|i| i.is_whitespace() || *i == ';')
            .count();
        let name: Vec<String> = self
            .take_while_ref(|i| !(i.is_ascii_control() || *i == ';' || *i == '<'))
            .collect::<String>()
            .split_terminator(' ')
            .map(|v| v.trim().to_string())
            .collect();

        if name.len() < 1 {
            return Err(self.error("Expected to find author name".to_string()));
        }
        if name.len() < 2 {
            return Err(self.error("Expected to find author's last name".to_string()));
        }

        let first_name = name[0].clone();

        let (middle_name, last_name) = if name.len() == 2 {
            (None, name[1].clone())
        } else {
            (
                Some(name[1].clone()),
                name.last()
                    .expect("Expected the name to have more than 2 words")
                    .to_string(),
            )
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
            .take_while_ref(|i| i.is_alphanumeric() || "-._~:/?#[]@!$&'()*+,;%=".contains(*i))
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
            Ok(v) => {
                self.drop()
                    .expect("Drop executed before pop on checkpoint iterator");
                Ok(v)
            }
        }
    }
}

impl<T> LineParser for CheckpointIterator<T>
where
    T: Iterator<Item = char>,
{
    fn parse_line(&mut self) -> Result<String, super::ParseError> {
        Ok(self
            .take_while(|i| !i.is_ascii_control())
            .collect::<String>())
    }
}
