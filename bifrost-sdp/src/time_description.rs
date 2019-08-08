use nom::IResult;

use crate::{Parse, Timing};

/// A parsed SDP time description, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5).
#[derive(Debug, PartialEq)]
pub struct TimeDescription {
    pub timing: Timing,
}

impl Parse for TimeDescription {
    fn parse(input: &str) -> IResult<&str, Self> {
        // t=  (time the session is active)
        // r=* (zero or more repeat times)

        let (rest, timing) = Parse::parse(input)?;

        Ok((rest, Self { timing }))
    }
}

#[cfg(test)]
mod tests {
    // TODO: add tests
}
