// Copyright 2020 - developers of the `grammers` project.
// Copyright 2021 - developers of the `tdlib-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use crate::errors::ParseError;
use crate::tl::{Category, Definition};

const DEFINITION_SEP: char = ';';
const FUNCTIONS_SEP: &str = "---functions---";
const TYPES_SEP: &str = "---types---";

/// An iterator over [Type Language] definitions.
///
/// [Type Language]: https://core.telegram.org/mtproto/TL
pub struct TlIterator {
    contents: String,
    index: usize,
    category: Category,
}

impl TlIterator {
    pub(crate) fn new(contents: String) -> Self {
        TlIterator {
            contents,
            index: 0,
            category: Category::Types,
        }
    }
}

impl Iterator for TlIterator {
    type Item = Result<Definition, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let definition = loop {
            if self.index >= self.contents.len() {
                return None;
            }

            let (end, is_empty) = {
                let mut chars = self.contents[self.index..].char_indices().peekable();
                let mut in_comment = false;
                let mut is_empty = true;

                loop {
                    if let Some((i, c)) = chars.next() {
                        if !in_comment && c == '/' {
                            if let Some((_, pc)) = chars.peek() {
                                if *pc == '/' {
                                    in_comment = true;
                                }
                            }
                        } else if in_comment && c == '\n' {
                            in_comment = false;
                        }

                        if !in_comment {
                            if !c.is_whitespace() {
                                is_empty = false;
                            }

                            if c == DEFINITION_SEP {
                                break (self.index + i, is_empty);
                            }
                        }
                    } else {
                        break (self.contents.len(), is_empty);
                    }
                }
            };

            let definition = self.contents[self.index..end].trim();
            self.index = end + DEFINITION_SEP.len_utf8();

            if !is_empty {
                break definition;
            }
        };

        // Get rid of the leading separator and adjust category
        let definition = if definition.starts_with("---") {
            if let Some(definition) = definition.strip_prefix(FUNCTIONS_SEP) {
                self.category = Category::Functions;
                definition.trim()
            } else if let Some(definition) = definition.strip_prefix(TYPES_SEP) {
                self.category = Category::Types;
                definition.trim()
            } else {
                return Some(Err(ParseError::UnknownSeparator));
            }
        } else {
            definition
        };

        // Yield the fixed definition
        Some(match definition.parse::<Definition>() {
            Ok(mut d) => {
                d.category = self.category;
                Ok(d)
            }
            x => x,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ParseError;

    #[test]
    fn parse_bad_separator() {
        let mut it = TlIterator::new("---foo---".into());
        assert_eq!(it.next(), Some(Err(ParseError::UnknownSeparator)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn parse_file() {
        let mut it = TlIterator::new(
            "
            // leading; comment
            first = t; // inline comment
            second and bad;
            third = t;
            // trailing comment
        "
            .into(),
        );

        assert_eq!(it.next().unwrap().unwrap().name, "first");
        assert!(it.next().unwrap().is_err());
        assert_eq!(it.next().unwrap().unwrap().name, "third");
        assert_eq!(it.next(), None);
    }
}
