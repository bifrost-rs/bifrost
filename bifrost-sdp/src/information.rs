use nom::combinator::map;
use nom::IResult;

use crate::util;
use crate::Parse;

// https://tools.ietf.org/html/rfc4566#section-5.4
// i=<session description>
#[derive(Debug, PartialEq)]
pub struct Information(pub String);

impl Parse for Information {
    fn parse(input: &str) -> IResult<&str, Information> {
        map(util::parse_single_field_line("i="), |value| {
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
