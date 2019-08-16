use nom::IResult;
use vec1::Vec1;

use crate::{Parse, RepeatTimes, Timing};

/// A parsed SDP time description, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5).
#[derive(Debug, PartialEq)]
pub struct TimeDescription {
    pub timing: Timing,
    pub repeat_times: Vec1<RepeatTimes>,
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
    // TODO: add tests
}
