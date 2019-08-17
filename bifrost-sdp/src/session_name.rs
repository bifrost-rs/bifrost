use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::IResult;

use crate::{util, Parse};

/// A parsed session name line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.3).
#[derive(Debug, PartialEq)]
pub struct SessionName<'a>(pub Cow<'a, str>);

impl<'a> Parse<'a> for SessionName<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // s=<session name>
        let (rest, _) = tag("s=")(input)?;
        let (rest, value) = util::parse_line(rest)?;
        Ok((rest, Self(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_1() {
        let s = "s=test\r\nrest\n";
        let (rest, SessionName(session_name)) = SessionName::parse(s).unwrap();
        assert_eq!(rest, "rest\n");
        assert_eq!(session_name, "test");
    }

    #[test]
    fn test_valid_2() {
        let s = "s= \nmore\r\n";
        let (rest, SessionName(session_name)) = SessionName::parse(s).unwrap();
        assert_eq!(rest, "more\r\n");
        assert_eq!(session_name, " ");
    }

    #[test]
    fn test_empty() {
        assert!(SessionName::parse("s=\r\n").is_err());
    }
}
