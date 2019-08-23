use std::fmt;

use http::Uri as HttpUri;
use nom::combinator::{map, map_res};
use nom::IResult;

use crate::{util, Parse};

/// A parsed URI line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.5).
#[derive(Clone, Debug, PartialEq)]
pub struct Uri(pub HttpUri);

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "u={}\r", self.0)
    }
}

impl Parse for Uri {
    fn parse(input: &str) -> IResult<&str, Self> {
        // u=<uri>
        map(map_res(util::parse_nonempty_line("u="), str::parse), Self)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn test_valid() {
        let uri_str = "http://www.example.com/seminars/sdp.pdf";
        assert_parse_display(
            &format!("u={}\r\nrest\n", uri_str),
            "rest\n",
            &Uri(HttpUri::from_static(uri_str)),
            &format!("u={}\r\n", uri_str),
        );
    }

    #[test]
    fn test_invalid() {
        assert_err::<Uri>("u=\r\n");
        assert_err::<Uri>("u= \r\n");
    }
}
