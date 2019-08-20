use nom::IResult;

use crate::{Parse, RepeatTimes, Timing};

/// A parsed SDP time description, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeDescription {
    pub timing: Timing,
    pub repeat_times: Vec<RepeatTimes>,
}

impl Parse for TimeDescription {
    fn parse(input: &str) -> IResult<&str, Self> {
        // t=  (time the session is active)
        // r=* (zero or more repeat times)

        let (rest, timing) = Parse::parse(input)?;
        let (rest, repeat_times) = Parse::parse(rest)?;

        Ok((
            rest,
            Self {
                timing,
                repeat_times,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use vec1::vec1;

    use super::*;
    use crate::{Duration, Instant};

    #[test]
    fn test_valid() {
        let s1 = "t=3034423619 3042462419\r\nmore\n";
        assert_eq!(
            TimeDescription::parse(s1),
            Ok((
                "more\n",
                TimeDescription {
                    timing: Timing {
                        start_time: Instant::from_secs(3_034_423_619),
                        stop_time: Instant::from_secs(3_042_462_419),
                    },
                    repeat_times: vec![],
                }
            ))
        );

        let s2 = "t=3034423619 3042462419\r\n\
                  r=604800 3600 0 90000\r\nmore\r\n";
        assert_eq!(
            TimeDescription::parse(s2),
            Ok((
                "more\r\n",
                TimeDescription {
                    timing: Timing {
                        start_time: Instant::from_secs(3_034_423_619),
                        stop_time: Instant::from_secs(3_042_462_419),
                    },
                    repeat_times: vec![RepeatTimes {
                        interval: Duration::from_secs(604_800),
                        duration: Duration::from_secs(3600),
                        offsets: vec1![Duration::from_secs(0), Duration::from_secs(90000)],
                    }],
                }
            ))
        );

        let s3 = "t=3034423619 3042462419\r\n\
                  r=604800 3600 0 90000\r\n\
                  more\r\n";
        assert_eq!(
            TimeDescription::parse(s3),
            Ok((
                "more\r\n",
                TimeDescription {
                    timing: Timing {
                        start_time: Instant::from_secs(3_034_423_619),
                        stop_time: Instant::from_secs(3_042_462_419),
                    },
                    repeat_times: vec![RepeatTimes {
                        interval: Duration::from_secs(604_800),
                        duration: Duration::from_secs(3600),
                        offsets: vec1![Duration::from_secs(0), Duration::from_secs(90000)],
                    }],
                }
            ))
        );

        let s4 = "t=3034423619 3042462419\r\n\
                  r=604800 3600 0 90000\r\n\
                  r=604801 3601 1 90001\r\n\
                  more\r\n";
        assert_eq!(
            TimeDescription::parse(s4),
            Ok((
                "more\r\n",
                TimeDescription {
                    timing: Timing {
                        start_time: Instant::from_secs(3_034_423_619),
                        stop_time: Instant::from_secs(3_042_462_419),
                    },
                    repeat_times: vec![
                        RepeatTimes {
                            interval: Duration::from_secs(604_800),
                            duration: Duration::from_secs(3600),
                            offsets: vec1![Duration::from_secs(0), Duration::from_secs(90000)],
                        },
                        RepeatTimes {
                            interval: Duration::from_secs(604_801),
                            duration: Duration::from_secs(3601),
                            offsets: vec1![Duration::from_secs(1), Duration::from_secs(90001)],
                        }
                    ],
                }
            ))
        );
    }

    #[test]
    fn test_invalid() {
        let s = "r=604800 3600 0 90000\r\nmore";
        assert!(TimeDescription::parse(s).is_err());
    }
}
