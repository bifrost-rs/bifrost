use nom::combinator::map;
use nom::IResult;

use crate::util;
use crate::Parse;

/// A parsed session name line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.3).
#[derive(Debug, PartialEq)]
pub struct SessionName(pub String);

impl Parse for SessionName {
    fn parse(input: &str) -> IResult<&str, SessionName> {
        // s=<session name>
        map(util::parse_single_field_line("s="), |value| {
            SessionName(value.to_owned())
        })(input)
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
