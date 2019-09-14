use crate::{Duration, Instant, Parse};
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_nonempty_list;
use nom::IResult;
use std::fmt;
use vec1::Vec1;

/// A parsed time zones line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.11).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeZones(pub Vec1<TimeZone>);

impl fmt::Display for TimeZones {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first = self.0.first();
        write!(
            f,
            "z={} {}",
            first.adjustment_time.as_secs(),
            first.offset.as_secs()
        )?;
        self.0.iter().skip(1).try_for_each(|tz| {
            write!(
                f,
                " {} {}",
                tz.adjustment_time.as_secs(),
                tz.offset.as_secs()
            )
        })?;
        writeln!(f, "\r")
    }
}

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
#[derive(Clone, Debug, PartialEq)]
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
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};
    use vec1::vec1;

    #[test]
    fn valid_time_zone() {
        assert_eq!(
            TimeZone::parse("2882844526 -1h\r\n"),
            Ok((
                "\r\n",
                TimeZone {
                    adjustment_time: Instant::from_secs(2_882_844_526),
                    offset: Duration::from_hours(-1),
                }
            ))
        );

        assert_eq!(
            TimeZone::parse("2898848070 0 hello"),
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
    fn invalid_time_zone() {
        assert_err::<TimeZone>("foo");
        assert_err::<TimeZone>("2 hello");
    }

    #[test]
    fn valid_time_zones() {
        assert_parse_display(
            "z=2882844526 -1h\r\nmore",
            "more",
            &TimeZones(vec1![TimeZone {
                adjustment_time: Instant::from_secs(2_882_844_526),
                offset: Duration::from_hours(-1),
            }]),
            "z=2882844526 -3600\r\n",
        );

        assert_parse_display(
            "z=2882844526 -1h 2898848070 0\r\nmore\n",
            "more\n",
            &TimeZones(vec1![
                TimeZone {
                    adjustment_time: Instant::from_secs(2_882_844_526),
                    offset: Duration::from_hours(-1),
                },
                TimeZone {
                    adjustment_time: Instant::from_secs(2_898_848_070),
                    offset: Duration::from_secs(0),
                }
            ]),
            "z=2882844526 -3600 2898848070 0\r\n",
        );

        assert_parse_display(
            "z=2882844526 -1h 2898848070 0 42 +25d\r\nmore\n",
            "more\n",
            &TimeZones(vec1![
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
            ]),
            "z=2882844526 -3600 2898848070 0 42 2160000\r\n",
        );
    }

    #[test]
    fn invalid_time_zones() {
        assert_err::<TimeZones>("z=1\r\n");
        assert_err::<TimeZones>("z=1 2 3\r\n");
        assert_err::<TimeZones>("z=1 2 3 4 5\r\n");
        assert_err::<TimeZones>("z=\r\n");
        assert_err::<TimeZones>("z=s 1\r\n");
        assert_err::<TimeZones>("z= 1 2\r\n");
        assert_err::<TimeZones>("z=1 2 \r\n");
    }
}
