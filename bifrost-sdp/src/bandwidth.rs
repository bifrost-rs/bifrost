use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::IResult;

use crate::{util, Parse};

/// A parsed bandwidth line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.8).
#[derive(Debug, PartialEq)]
pub struct Bandwidth<'a> {
    pub experimental: bool,
    pub bwtype: Cow<'a, str>,
    pub bandwidth: u64,
}

impl<'a> Parse<'a> for Bandwidth<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // b=<bwtype>:<bandwidth>
        let (rest, _) = tag("b=")(input)?;

        let experimental = rest.starts_with("X-");
        let rest = if experimental { &rest[2..] } else { rest };

        let (rest, bwtype) = alphanumeric1(rest)?;
        let (rest, _) = tag(":")(rest)?;
        let (rest, bandwidth) = util::parse_field(rest)?;
        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Self {
                experimental,
                bwtype: bwtype.into(),
                bandwidth,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s = "b=CT:256\r\nmore\r\n";
        let (rest, bandwidth) = Bandwidth::parse(s).unwrap();
        assert_eq!(rest, "more\r\n");
        assert_eq!(
            bandwidth,
            Bandwidth {
                experimental: false,
                bwtype: "CT".into(),
                bandwidth: 256,
            }
        )
    }

    #[test]
    fn test_experimental() {
        let s = "b=X-AB:512\r\n";
        let (rest, bandwidth) = Bandwidth::parse(s).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            bandwidth,
            Bandwidth {
                experimental: true,
                bwtype: "AB".into(),
                bandwidth: 512,
            }
        )
    }

    #[test]
    fn test_invalid() {
        assert!(Bandwidth::parse("b=A-AB:512\r\n").is_err());
        assert!(Bandwidth::parse("b=AB:foo\r\n").is_err());
    }
}
