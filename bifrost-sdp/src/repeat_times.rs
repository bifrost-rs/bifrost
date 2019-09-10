use crate::{Duration, Parse};
use nom::{
    bytes::complete::tag, character::complete::line_ending, multi::separated_nonempty_list, IResult,
};
use std::fmt;
use vec1::Vec1;

/// A parsed repeat times line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.10).
#[derive(Clone, Debug, PartialEq)]
pub struct RepeatTimes {
    pub interval: Duration,
    pub duration: Duration,
    pub offsets: Vec1<Duration>,
}

impl fmt::Display for RepeatTimes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "r={} {}",
            self.interval.as_secs(),
            self.duration.as_secs()
        )?;
        self.offsets
            .iter()
            .try_for_each(|x| write!(f, " {}", x.as_secs()))?;
        writeln!(f, "\r")
    }
}

impl Parse for RepeatTimes {
    fn parse(input: &str) -> IResult<&str, Self> {
        // r=<repeat interval> <active duration> <offsets from start-time>
        //
        // Each field is an integer, optionally followed by a unit specification
        // character:
        //   d - days (86400 seconds)
        //   h - hours (3600 seconds)
        //   m - minutes (60 seconds)
        //   s - seconds (allowed for completeness)

        let (rest, _) = tag("r=")(input)?;

        let (rest, interval) = Parse::parse(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, duration) = Parse::parse(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, offsets) = separated_nonempty_list(tag(" "), Parse::parse)(rest)?;
        let offsets = Vec1::try_from_vec(offsets).unwrap();

        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Self {
                interval,
                duration,
                offsets,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};
    use vec1::vec1;

    #[test]
    fn valid() {
        assert_parse_display(
            "r=1 2 -3\r\nmore",
            "more",
            &RepeatTimes {
                interval: Duration::from_secs(1),
                duration: Duration::from_secs(2),
                offsets: vec1![Duration::from_secs(-3)],
            },
            "r=1 2 -3\r\n",
        );

        assert_parse_display(
            "r=1 2 +3 -4\r\nmore",
            "more",
            &RepeatTimes {
                interval: Duration::from_secs(1),
                duration: Duration::from_secs(2),
                offsets: vec1![Duration::from_secs(3), Duration::from_secs(-4)],
            },
            "r=1 2 3 -4\r\n",
        );

        assert_parse_display(
            "r=1 2 3 4 5\r\nmore",
            "more",
            &RepeatTimes {
                interval: Duration::from_secs(1),
                duration: Duration::from_secs(2),
                offsets: vec1![
                    Duration::from_secs(3),
                    Duration::from_secs(4),
                    Duration::from_secs(5)
                ],
            },
            "r=1 2 3 4 5\r\n",
        );
    }

    #[test]
    fn valid_units() {
        assert_parse_display(
            "r=1d -2h 3m\r\nmore",
            "more",
            &RepeatTimes {
                interval: Duration::from_days(1),
                duration: Duration::from_hours(-2),
                offsets: vec1![Duration::from_mins(3)],
            },
            "r=86400 -7200 180\r\n",
        );

        assert_parse_display(
            "r=+1h +2m 3s -4d\r\nmore",
            "more",
            &RepeatTimes {
                interval: Duration::from_hours(1),
                duration: Duration::from_mins(2),
                offsets: vec1![Duration::from_secs(3), Duration::from_days(-4)],
            },
            "r=3600 120 3 -345600\r\n",
        );

        assert_parse_display(
            "r=1m 2 +3h -4s 5d\r\nmore",
            "more",
            &RepeatTimes {
                interval: Duration::from_mins(1),
                duration: Duration::from_secs(2),
                offsets: vec1![
                    Duration::from_hours(3),
                    Duration::from_secs(-4),
                    Duration::from_days(5)
                ],
            },
            "r=60 2 10800 -4 432000\r\n",
        );

        assert_parse_display(
            "r=-7d 1h 0 +25h\r\nrest",
            "rest",
            &RepeatTimes {
                interval: Duration::from_days(-7),
                duration: Duration::from_hours(1),
                offsets: vec1![Duration::from_secs(0), Duration::from_hours(25)],
            },
            "r=-604800 3600 0 90000\r\n",
        )
    }

    #[test]
    fn invalid() {
        assert_err::<RepeatTimes>("r=1 2\r\nmore");
        assert_err::<RepeatTimes>("r=1 2  3\r\nmore");
        assert_err::<RepeatTimes>("r=1 2 3 \r\nmore");
        assert_err::<RepeatTimes>("r= 1 2 3\r\nmore");
    }

    #[test]
    fn invalid_units() {
        assert_err::<RepeatTimes>("r=1x 2 3\r\nmore");
        assert_err::<RepeatTimes>("r=1d 2h 3x\r\nmore");
        assert_err::<RepeatTimes>("r=s 2 3\r\nmore");
    }
}
