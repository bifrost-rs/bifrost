use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_nonempty_list;
use nom::IResult;
use vec1::Vec1;

use crate::{Duration, Instant, Parse};

/// A parsed time zones line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.11).
#[derive(Debug, PartialEq)]
pub struct TimeZones(pub Vec1<TimeZone>);

impl Parse for TimeZones {
    fn parse(input: &str) -> IResult<&str, Self> {
        // z=<adjustment time> <offset> <adjustment time> <offset> ....
        let (rest, _) = tag("z=")(input)?;
        let (rest, time_zones) = separated_nonempty_list(tag(" "), Parse::parse)(rest)?;
        let time_zones = Vec1::try_from_vec(time_zones).unwrap();
        let (rest, _) = line_ending(rest)?;

        Ok((rest, Self(time_zones)))
    }
}

/// One pair of <adjustment time> and <offset> in a time zones line.
#[derive(Debug, PartialEq)]
pub struct TimeZone {
    pub adjustment_time: Instant,
    pub offset: Duration,
}

impl Parse for TimeZone {
    fn parse(input: &str) -> IResult<&str, Self> {
        // <adjustment time> <offset>
        let (rest, adjustment_time) = Parse::parse(input)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, offset) = Parse::parse(rest)?;

        Ok((
            rest,
            Self {
                adjustment_time,
                offset,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use vec1::vec1;

    use super::*;

    #[test]
    fn test_valid_time_zone() {
        let s1 = "2882844526 -1h\r\n";
        assert_eq!(
            TimeZone::parse(s1),
            Ok((
                "\r\n",
                TimeZone {
                    adjustment_time: Instant::from_secs(2_882_844_526),
                    offset: Duration::from_hours(-1),
                }
            ))
        );

        let s2 = "2898848070 0 hello";
        assert_eq!(
            TimeZone::parse(s2),
            Ok((
                " hello",
                TimeZone {
                    adjustment_time: Instant::from_secs(2_898_848_070),
                    offset: Duration::from_secs(0),
                }
            ))
        );
    }

    #[test]
    fn test_invalid_time_zone() {
        assert!(TimeZone::parse("foo").is_err());
        assert!(TimeZone::parse("2 hello").is_err());
    }

    #[test]
    fn test_valid_time_zones() {
        let s1 = "z=2882844526 -1h\r\nmore";
        assert_eq!(
            TimeZones::parse(s1),
            Ok((
                "more",
                TimeZones(vec1![TimeZone {
                    adjustment_time: Instant::from_secs(2_882_844_526),
                    offset: Duration::from_hours(-1),
                }])
            ))
        );

        let s2 = "z=2882844526 -1h 2898848070 0\r\nmore\n";
        assert_eq!(
            TimeZones::parse(s2),
            Ok((
                "more\n",
                TimeZones(vec1![
                    TimeZone {
                        adjustment_time: Instant::from_secs(2_882_844_526),
                        offset: Duration::from_hours(-1),
                    },
                    TimeZone {
                        adjustment_time: Instant::from_secs(2_898_848_070),
                        offset: Duration::from_secs(0),
                    }
                ])
            ))
        );

        let s3 = "z=2882844526 -1h 2898848070 0 42 +25d\r\nmore\n";
        assert_eq!(
            TimeZones::parse(s3),
            Ok((
                "more\n",
                TimeZones(vec1![
                    TimeZone {
                        adjustment_time: Instant::from_secs(2_882_844_526),
                        offset: Duration::from_hours(-1),
                    },
                    TimeZone {
                        adjustment_time: Instant::from_secs(2_898_848_070),
                        offset: Duration::from_secs(0),
                    },
                    TimeZone {
                        adjustment_time: Instant::from_secs(42),
                        offset: Duration::from_days(25),
                    }
                ])
            ))
        );
    }

    #[test]
    fn test_invalid_time_zones() {
        assert!(TimeZones::parse("z=1\r\n").is_err());
        assert!(TimeZones::parse("z=1 2 3\r\n").is_err());
        assert!(TimeZones::parse("z=1 2 3 4 5\r\n").is_err());
        assert!(TimeZones::parse("z=\r\n").is_err());
        assert!(TimeZones::parse("z=s 1\r\n").is_err());
        assert!(TimeZones::parse("z= 1 2\r\n").is_err());
        assert!(TimeZones::parse("z=1 2 \r\n").is_err());
    }
}
