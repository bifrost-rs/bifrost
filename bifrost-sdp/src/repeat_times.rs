use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_nonempty_list;
use nom::IResult;
use vec1::Vec1;

use crate::{Duration, Parse};

/// A parsed repeat times field, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.10).
#[derive(Debug, PartialEq)]
pub struct RepeatTimes {
    pub interval: Duration,
    pub duration: Duration,
    pub offsets: Vec1<Duration>,
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
    use vec1::vec1;

    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(
            RepeatTimes::parse("r=1 2 -3\r\nmore"),
            Ok((
                "more",
                RepeatTimes {
                    interval: Duration::from_secs(1),
                    duration: Duration::from_secs(2),
                    offsets: vec1![Duration::from_secs(-3)],
                }
            ))
        );

        assert_eq!(
            RepeatTimes::parse("r=1 2 +3 -4\r\nmore"),
            Ok((
                "more",
                RepeatTimes {
                    interval: Duration::from_secs(1),
                    duration: Duration::from_secs(2),
                    offsets: vec1![Duration::from_secs(3), Duration::from_secs(-4)],
                }
            ))
        );

        assert_eq!(
            RepeatTimes::parse("r=1 2 3 4 5\r\nmore"),
            Ok((
                "more",
                RepeatTimes {
                    interval: Duration::from_secs(1),
                    duration: Duration::from_secs(2),
                    offsets: vec1![
                        Duration::from_secs(3),
                        Duration::from_secs(4),
                        Duration::from_secs(5)
                    ],
                }
            ))
        );
    }

    #[test]
    fn test_valid_units() {
        assert_eq!(
            RepeatTimes::parse("r=1d -2h 3m\r\nmore"),
            Ok((
                "more",
                RepeatTimes {
                    interval: Duration::from_days(1),
                    duration: Duration::from_hours(-2),
                    offsets: vec1![Duration::from_mins(3)],
                }
            ))
        );

        assert_eq!(
            RepeatTimes::parse("r=+1h +2m 3s -4d\r\nmore"),
            Ok((
                "more",
                RepeatTimes {
                    interval: Duration::from_hours(1),
                    duration: Duration::from_mins(2),
                    offsets: vec1![Duration::from_secs(3), Duration::from_days(-4)],
                }
            ))
        );

        assert_eq!(
            RepeatTimes::parse("r=1m 2 +3h -4s 5d\r\nmore"),
            Ok((
                "more",
                RepeatTimes {
                    interval: Duration::from_mins(1),
                    duration: Duration::from_secs(2),
                    offsets: vec1![
                        Duration::from_hours(3),
                        Duration::from_secs(-4),
                        Duration::from_days(5)
                    ],
                }
            ))
        );
    }

    #[test]
    fn test_invalid() {
        assert!(RepeatTimes::parse("r=1 2\r\nmore").is_err());
        assert!(RepeatTimes::parse("r=1 2  3\r\nmore").is_err());
        assert!(RepeatTimes::parse("r=1 2 3 \r\nmore").is_err());
        assert!(RepeatTimes::parse("r= 1 2 3\r\nmore").is_err());
    }

    #[test]
    fn test_invalid_units() {
        assert!(RepeatTimes::parse("r=1x 2 3\r\nmore").is_err());
        assert!(RepeatTimes::parse("r=1d 2h 3x\r\nmore").is_err());
        assert!(RepeatTimes::parse("r=s 2 3\r\nmore").is_err());
    }
}
