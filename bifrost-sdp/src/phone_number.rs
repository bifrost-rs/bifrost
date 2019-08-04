use nom::combinator::map;
use nom::IResult;

use crate::util;
use crate::Parse;

/// A parsed phone number line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.6).
#[derive(Debug, PartialEq)]
pub struct PhoneNumber(pub String);

impl Parse for PhoneNumber {
    fn parse(input: &str) -> IResult<&str, Self> {
        // p=<phone-number>
        map(util::parse_nonempty_line("p="), |value| {
            Self(value.to_owned())
        })(input)
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
        assert_eq!(phone_number, PhoneNumber(phone_number_str.to_owned()));
    }

    #[test]
    fn test_invalid() {
        assert!(PhoneNumber::parse("p=\r\n").is_err());
        assert!(PhoneNumber::parse("x=+1 617 555-6011\r\n").is_err());
    }
}
