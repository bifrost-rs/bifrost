use nom::bytes::complete::tag;
use nom::IResult;

use crate::util;
use crate::Parse;

/// A parsed timing line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.9).
#[derive(Debug, PartialEq)]
pub struct Timing {
    pub start_time: u64,
    pub stop_time: u64,
}

impl Parse for Timing {
    fn parse(input: &str) -> IResult<&str, Self> {
        // t=<start-time> <stop-time>
        let (rest, _) = tag("t=")(input)?;
        let (rest, start_time) = util::parse_field(rest)?;
        let (rest, stop_time) = util::parse_last_field(rest)?;

        Ok((
            rest,
            Self {
                start_time,
                stop_time,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s = "t=123 456\nmore\r\n";
        let (rest, timing) = Timing::parse(s).unwrap();
        assert_eq!(rest, "more\r\n");
        assert_eq!(
            timing,
            Timing {
                start_time: 123,
                stop_time: 456
            }
        );
    }

    #[test]
    fn test_invalid() {
        assert!(Timing::parse("t=123\r\n").is_err());
        assert!(Timing::parse("t=foo 456\r\n").is_err());
        assert!(Timing::parse("t=123 foo\r\n").is_err());
        assert!(Timing::parse("t=123 456 789\r\n").is_err());
    }
}
