use std::fmt;

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

impl fmt::Display for Timing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "t={} {}\r",
            self.start_time.as_secs(),
            self.stop_time.as_secs()
        )
    }
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
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn test_valid() {
        assert_parse_display(
            "t=123 456\nmore\r\n",
            "more\r\n",
            &Timing {
                start_time: Instant::from_secs(123),
                stop_time: Instant::from_secs(456),
            },
            "t=123 456\r\n",
        );
    }

    #[test]
    fn test_invalid() {
        assert_err::<Timing>("t=123\r\n");
        assert_err::<Timing>("t=foo 456\r\n");
        assert_err::<Timing>("t=123 foo\r\n");
        assert_err::<Timing>("t=123 456 789\r\n");
    }
}
