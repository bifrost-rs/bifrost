use nom::combinator::map_res;
use nom::IResult;

use crate::util;
use crate::Parse;

pub use url::Url;

impl Parse for Url {
    fn parse(input: &str) -> IResult<&str, Url> {
        map_res(util::parse_single_field_line("u="), Url::parse)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(<Url as Parse>::parse("u=http://www.example.com/seminars/sdp.pdf\r\n").is_ok());
    }

    #[test]
    fn test_invalid() {
        assert!(<Url as Parse>::parse("u=foo\r\n").is_err());
        assert!(<Url as Parse>::parse("u=\r\n").is_err());
    }
}
