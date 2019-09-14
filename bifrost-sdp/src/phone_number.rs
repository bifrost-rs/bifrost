use crate::util;
use crate::Parse;
use nom::combinator::map;
use nom::IResult;
use std::fmt;

/// A parsed phone number line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.6).
#[derive(Clone, Debug, PartialEq)]
pub struct PhoneNumber(pub String);

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "p={}\r", self.0)
    }
}

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
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn valid() {
        let phone_number = "+1 617 555-6011";
        assert_parse_display(
            &format!("p={}\n\nresto\r\n", phone_number),
            "\nresto\r\n",
            &PhoneNumber(phone_number.to_owned()),
            &format!("p={}\r\n", phone_number),
        );
    }

    #[test]
    fn invalid() {
        assert_err::<PhoneNumber>("p=\r\n");
        assert_err::<PhoneNumber>("x=+1 617 555-6011\r\n");
    }
}
