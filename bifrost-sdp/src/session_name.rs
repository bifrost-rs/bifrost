use std::fmt;

use nom::{combinator::map, IResult};

use crate::{util, Parse};

/// A parsed session name line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.3).
#[derive(Clone, Debug, PartialEq)]
pub struct SessionName(pub String);

impl fmt::Display for SessionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "s={}\r", self.0)
    }
}

impl Parse for SessionName {
    fn parse(input: &str) -> IResult<&str, Self> {
        // s=<session name>
        map(util::parse_nonempty_line("s="), |value| {
            Self(value.to_owned())
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn test_valid() {
        assert_parse_display(
            "s=test\r\nrest\n",
            "rest\n",
            &SessionName("test".to_owned()),
            "s=test\r\n",
        );

        assert_parse_display(
            "s= \nmore\r\n",
            "more\r\n",
            &SessionName(" ".to_owned()),
            "s= \r\n",
        );
    }

    #[test]
    fn test_invalid() {
        assert_err::<SessionName>("s=\r\n");
    }
}
