// Copyright 2020 - developers of the `grammers` project.
// Copyright 2022 - developers of the `tdlib-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::fmt;
use std::str::FromStr;

use crate::errors::ParamParseError;

/// The type of a definition or a parameter.
#[derive(Debug, PartialEq)]
pub struct Type {
    /// The name of the type.
    pub name: String,

    /// Whether this type is bare or boxed.
    pub bare: bool,

    /// If the type has a generic argument, which is its type.
    pub generic_arg: Option<Box<Type>>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(generic_arg) = &self.generic_arg {
            write!(f, "<{}>", generic_arg)?;
        }
        Ok(())
    }
}

impl FromStr for Type {
    type Err = ParamParseError;

    /// Parses a type.
    ///
    /// # Examples
    ///
    /// ```
    /// use tdlib_tl_parser::tl::Type;
    ///
    /// assert!("vector<int>".parse::<Type>().is_ok());
    /// ```
    fn from_str(ty: &str) -> Result<Self, Self::Err> {
        // Parse `type<generic_arg>`
        let (ty, generic_arg) = if let Some(pos) = ty.find('<') {
            if !ty.ends_with('>') {
                return Err(ParamParseError::InvalidGeneric);
            }
            (
                &ty[..pos],
                Some(Box::new(Type::from_str(&ty[pos + 1..ty.len() - 1])?)),
            )
        } else {
            (ty, None)
        };

        if ty.is_empty() {
            return Err(ParamParseError::Empty);
        }

        // Safe to unwrap because we just checked is not empty
        let bare = ty.chars().next().unwrap().is_ascii_lowercase();

        Ok(Self {
            name: ty.into(),
            bare,
            generic_arg,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_empty_simple() {
        assert_eq!(Type::from_str(""), Err(ParamParseError::Empty));
    }

    #[test]
    fn check_simple() {
        assert_eq!(
            Type::from_str("foo"),
            Ok(Type {
                name: "foo".into(),
                bare: true,
                generic_arg: None,
            })
        );
    }

    #[test]
    fn check_empty() {
        assert_eq!(Type::from_str(""), Err(ParamParseError::Empty));
    }

    #[test]
    fn check_bare() {
        assert!(match Type::from_str("foo") {
            Ok(Type { bare: true, .. }) => true,
            _ => false,
        });
        assert!(match Type::from_str("Foo") {
            Ok(Type { bare: false, .. }) => true,
            _ => false,
        });
    }

    #[test]
    fn check_generic_arg() {
        assert!(match Type::from_str("foo") {
            Ok(Type {
                generic_arg: None, ..
            }) => true,
            _ => false,
        });
        assert!(match Type::from_str("foo<bar>") {
            Ok(Type {
                generic_arg: Some(x),
                ..
            }) => *x == "bar".parse().unwrap(),
            _ => false,
        });
        assert!(match Type::from_str("foo<bar>") {
            Ok(Type {
                generic_arg: Some(x),
                ..
            }) => *x == "bar".parse().unwrap(),
            _ => false,
        });
        assert!(match Type::from_str("foo<bar<baz>>") {
            Ok(Type {
                generic_arg: Some(x),
                ..
            }) => *x == "bar<baz>".parse().unwrap(),
            _ => false,
        });
    }
}
