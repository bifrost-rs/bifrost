use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;

use crate::Parse;

/// A parsed protocal version line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.1).
#[derive(Debug, PartialEq)]
pub struct Version;

impl Parse for Version {
    fn parse(input: &str) -> IResult<&str, Version> {
        // v=0
        let (rest, _) = tag("v=0")(input)?;
        let (rest, _) = line_ending(rest)?;
        Ok((rest, Version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(Version::parse("v=0\r\n").is_ok());
        assert!(Version::parse("v=0\n").is_ok());
    }

    #[test]
    fn test_unsupported_version() {
        assert!(Version::parse("v=1\r\n").is_err());
    }

    #[test]
    fn test_bad_format() {
        assert!(Version::parse("v =0\r\n").is_err());
        assert!(Version::parse("v=0 \r\n").is_err());
        assert!(Version::parse("v=0\r").is_err());
        assert!(Version::parse("v=\r\n").is_err());
        assert!(Version::parse("v=0 1\r\n").is_err());
        assert!(Version::parse("v=x\r\n").is_err());
    }
}
