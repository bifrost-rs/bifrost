use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, not_line_ending};
use nom::combinator::opt;
use nom::IResult;

use crate::Parse;

/// A parsed attribute line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.13).
#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
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

    #[test]
    fn test_property_attrs() {
        assert_eq!(
            Attribute::parse("a=foo\r\nmore"),
            Ok((
                "more",
                Attribute {
                    name: "foo".to_owned(),
                    value: None,
                }
            ))
        );

        assert_eq!(
            Attribute::parse("a= f o o \r\nmore"),
            Ok((
                "more",
                Attribute {
                    name: " f o o ".to_owned(),
                    value: None,
                }
            ))
        );
    }

    #[test]
    fn test_value_attrs() {
        assert_eq!(
            Attribute::parse("a=foo:bar\r\nmore"),
            Ok((
                "more",
                Attribute {
                    name: "foo".to_owned(),
                    value: Some("bar".to_owned()),
                }
            ))
        );

        assert_eq!(
            Attribute::parse("a=foo:b:ar \r\nmore"),
            Ok((
                "more",
                Attribute {
                    name: "foo".to_owned(),
                    value: Some("b:ar ".to_owned()),
                }
            ))
        );

        assert_eq!(
            Attribute::parse("a=foo:::\r\nmore"),
            Ok((
                "more",
                Attribute {
                    name: "foo".to_owned(),
                    value: Some("::".to_owned()),
                }
            ))
        );

        assert_eq!(
            Attribute::parse("a=foo:\r\nmore"),
            Ok((
                "more",
                Attribute {
                    name: "foo".to_owned(),
                    value: Some("".to_owned()),
                }
            ))
        );
    }

    #[test]
    fn test_invalid() {
        assert!(Attribute::parse("a=\r\n").is_err());
        assert!(Attribute::parse("a=:\r\n").is_err());
        assert!(Attribute::parse("a=:x\r\n").is_err());
    }
}
