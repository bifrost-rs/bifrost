use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::IResult;

use crate::{util, Parse};

/// A parsed phone number line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.6).
#[derive(Debug, PartialEq)]
pub struct PhoneNumber<'a>(pub Cow<'a, str>);

impl<'a> Parse<'a> for PhoneNumber<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // p=<phone-number>
        let (rest, _) = tag("p=")(input)?;
        let (rest, value) = util::parse_line(rest)?;
        Ok((rest, Self(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let phone_number_str = "+1 617 555-6011";
        let line = format!("p={}\nresto\r\n", phone_number_str);
        let (rest, phone_number) = PhoneNumber::parse(&line).unwrap();
        assert_eq!(rest, "resto\r\n");
        assert_eq!(phone_number, PhoneNumber(phone_number_str.into()));
    }

    #[test]
    fn test_invalid() {
        assert!(PhoneNumber::parse("p=\r\n").is_err());
        assert!(PhoneNumber::parse("x=+1 617 555-6011\r\n").is_err());
    }
}
