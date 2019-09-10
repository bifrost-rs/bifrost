use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, not_line_ending},
    combinator::opt,
    IResult,
};
use std::fmt;

use crate::Parse;

/// A parsed attribute line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.13).
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => writeln!(f, "a={}:{}\r", self.name, value),
            None => writeln!(f, "a={}\r", self.name),
        }
    }
}

impl Parse for Attribute {
    fn parse(input: &str) -> IResult<&str, Self> {
        // a=<attribute>
        // a=<attribute>:<value>
        let (rest, _) = tag("a=")(input)?;
        let (rest, name) = is_not(":\r\n")(rest)?;
        let (rest, value) = opt(parse_value)(rest)?;
        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Self {
                name: name.to_owned(),
                value: value.map(String::from),
            },
        ))
    }
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    let (rest, _) = tag(":")(input)?;
    not_line_ending(rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn property_attrs() {
        assert_parse_display(
            "a=foo\r\nmore",
            "more",
            &Attribute {
                name: "foo".to_owned(),
                value: None,
            },
            "a=foo\r\n",
        );

        assert_parse_display(
            "a= f o o \r\nmore",
            "more",
            &Attribute {
                name: " f o o ".to_owned(),
                value: None,
            },
            "a= f o o \r\n",
        );
    }

    #[test]
    fn value_attrs() {
        assert_parse_display(
            "a=foo:bar\r\nmore",
            "more",
            &Attribute {
                name: "foo".to_owned(),
                value: Some("bar".to_owned()),
            },
            "a=foo:bar\r\n",
        );

        assert_parse_display(
            "a=foo:b:ar \r\nmore",
            "more",
            &Attribute {
                name: "foo".to_owned(),
                value: Some("b:ar ".to_owned()),
            },
            "a=foo:b:ar \r\n",
        );

        assert_parse_display(
            "a=foo:::\r\nmore",
            "more",
            &Attribute {
                name: "foo".to_owned(),
                value: Some("::".to_owned()),
            },
            "a=foo:::\r\n",
        );

        assert_parse_display(
            "a=foo:\r\nmore",
            "more",
            &Attribute {
                name: "foo".to_owned(),
                value: Some("".to_owned()),
            },
            "a=foo:\r\n",
        );
    }

    #[test]
    fn invalid() {
        assert_err::<Attribute>("a=\r\n");
        assert_err::<Attribute>("a=:\r\n");
        assert_err::<Attribute>("a=:x\r\n");
    }
}
