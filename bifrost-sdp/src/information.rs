use std::fmt;

use nom::combinator::map;
use nom::IResult;

use crate::{util, Parse};

/// A parsed information line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.4).
#[derive(Clone, Debug, PartialEq)]
pub struct Information(pub String);

impl fmt::Display for Information {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "i={}\r", self.0)
    }
}

impl Parse for Information {
    fn parse(input: &str) -> IResult<&str, Self> {
        // i=<session description>
        map(util::parse_nonempty_line("i="), |value| {
            Self(value.to_owned())
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::Information;
    use crate::test_util::assert_parse_display;

    #[test]
    fn test_valid() {
        assert_parse_display(
            "i=test info\nrest\n",
            "rest\n",
            &Information("test info".to_owned()),
            "i=test info\r\n",
        );
    }
}
