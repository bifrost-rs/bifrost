use nom::combinator::map;
use nom::IResult;

use crate::util;
use crate::Parse;

/// A parsed information line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.4).
#[derive(Debug, PartialEq)]
pub struct Information(pub String);

impl Parse for Information {
    fn parse(input: &str) -> IResult<&str, Information> {
        // i=<session description>
        map(util::parse_nonempty_line("i="), |value| {
            Information(value.to_owned())
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s = "i=test info\r\nrest\n";
        let (rest, Information(info)) = Information::parse(s).unwrap();
        assert_eq!(rest, "rest\n");
        assert_eq!(info, "test info");
    }
}
