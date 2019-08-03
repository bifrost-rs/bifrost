use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;

// https://tools.ietf.org/html/rfc4566#section-5.1
// v=0
#[derive(Debug, PartialEq)]
pub struct Version {}

impl Version {
    pub fn parse(input: &str) -> IResult<&str, Version> {
        let (input, _) = tag("v=0")(input)?;
        let (input, _) = line_ending(input)?;
        Ok((input, Version {}))
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
