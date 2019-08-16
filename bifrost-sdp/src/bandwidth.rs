use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::IResult;

use crate::{util, Parse};

/// A parsed bandwidth line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.8).
#[derive(Debug, PartialEq)]
pub struct Bandwidth {
    pub experimental: bool,
    pub bwtype: String,
    pub bandwidth: u64,
}

impl Parse for Bandwidth {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, _) = tag("b=")(input)?;

        let experimental = rest.starts_with("X-");
        let rest = if experimental { &rest[2..] } else { rest };

        let (rest, bwtype) = map(alphanumeric1, String::from)(rest)?;
        let (rest, _) = tag(":")(rest)?;
        let (rest, bandwidth) = util::parse_last_field(rest)?;

        Ok((
            rest,
            Self {
                experimental,
                bwtype,
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
                bwtype: "CT".to_owned(),
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
                bwtype: "AB".to_owned(),
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
