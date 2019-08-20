use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;

use crate::{Instant, Parse};

/// A parsed timing line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.9).
#[derive(Clone, Debug, PartialEq)]
pub struct Timing {
    pub start_time: Instant,
    pub stop_time: Instant,
}

impl Parse for Timing {
    fn parse(input: &str) -> IResult<&str, Self> {
        // t=<start-time> <stop-time>

        let (rest, _) = tag("t=")(input)?;

        let (rest, start_time) = Parse::parse(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, stop_time) = Parse::parse(rest)?;
        let (rest, _) = line_ending(rest)?;

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
                start_time: Instant::from_secs(123),
                stop_time: Instant::from_secs(456),
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
