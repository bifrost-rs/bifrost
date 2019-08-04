use http::Uri as HttpUri;
use nom::combinator::{map, map_res};
use nom::IResult;

use crate::util;
use crate::Parse;

/// A parsed URI line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.5).
#[derive(Debug, PartialEq)]
pub struct Uri(pub HttpUri);

impl Parse for Uri {
    fn parse(input: &str) -> IResult<&str, Uri> {
        // u=<uri>
        map(
            map_res(util::parse_single_field_line("u="), str::parse),
            Uri,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(Uri::parse("u=http://www.example.com/seminars/sdp.pdf\r\n").is_ok());
    }

    #[test]
    fn test_invalid() {
        assert!(Uri::parse("u=\r\n").is_err());
        assert!(Uri::parse("u= \r\n").is_err());
    }
}
