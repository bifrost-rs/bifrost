use http::Uri as HttpUri;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::IResult;

use crate::Parse;

/// A parsed URI line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.5).
#[derive(Debug, PartialEq)]
pub struct Uri(pub HttpUri);

impl<'a> Parse<'a> for Uri {
    fn parse(input: &str) -> IResult<&str, Self> {
        // u=<uri>
        let (rest, _) = tag("u=")(input)?;
        let (rest, uri) = map_res(is_not("\r\n"), str::parse)(rest)?;
        let (rest, _) = line_ending(rest)?;

        Ok((rest, Self(uri)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let uri_str = "http://www.example.com/seminars/sdp.pdf";
        let line = format!("u={}\r\nrest\n", uri_str);
        let (rest, uri) = Uri::parse(&line).unwrap();
        assert_eq!(rest, "rest\n");
        assert_eq!(uri, Uri(HttpUri::from_static(uri_str)));
    }

    #[test]
    fn test_invalid() {
        assert!(Uri::parse("u=\r\n").is_err());
        assert!(Uri::parse("u= \r\n").is_err());
    }
}
