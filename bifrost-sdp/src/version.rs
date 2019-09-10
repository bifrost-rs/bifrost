use crate::Parse;
use nom::{bytes::complete::tag, character::complete::line_ending, IResult};
use std::fmt;

/// A parsed protocal version line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.1).
#[derive(Clone, Debug, PartialEq)]
pub struct Version;

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "v=0\r")
    }
}

impl Parse for Version {
    fn parse(input: &str) -> IResult<&str, Self> {
        // v=0
        let (rest, _) = tag("v=0")(input)?;
        let (rest, _) = line_ending(rest)?;
        Ok((rest, Self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn valid() {
        assert_parse_display("v=0\r\nmore", "more", &Version, "v=0\r\n");
        assert_parse_display("v=0\nrest\n", "rest\n", &Version, "v=0\r\n");
    }

    #[test]
    fn unsupported_version() {
        assert_err::<Version>("v=1\r\n");
    }

    #[test]
    fn bad_format() {
        assert_err::<Version>("v =0\r\n");
        assert_err::<Version>("v=0 \r\n");
        assert_err::<Version>("v=0\r");
        assert_err::<Version>("v=\r\n");
        assert_err::<Version>("v=0 1\r\n");
        assert_err::<Version>("v=x\r\n");
    }
}
