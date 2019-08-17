use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::IResult;

use crate::{util, Parse};

/// A parsed information line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.4).
#[derive(Debug, PartialEq)]
pub struct Information<'a>(pub Cow<'a, str>);

impl<'a> Parse<'a> for Information<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // i=<session description>
        let (rest, _) = tag("i=")(input)?;
        let (rest, value) = util::parse_line(rest)?;
        Ok((rest, Self(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s1 = "i=test info\r\nrest\n";
        assert_eq!(
            Information::parse(s1),
            Ok(("rest\n", Information("test info".into())))
        );

        let s2 = "i=more test info\r\nmore\n".to_owned();
        assert_eq!(
            Information::parse(&s2),
            Ok(("more\n", Information("more test info".into())))
        );
    }
}
