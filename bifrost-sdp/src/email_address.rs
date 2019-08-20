use nom::combinator::map;
use nom::IResult;

use crate::{util, Parse};

/// A parsed email address line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.6).
#[derive(Clone, Debug, PartialEq)]
pub struct EmailAddress(pub String);

impl Parse for EmailAddress {
    fn parse(input: &str) -> IResult<&str, Self> {
        // e=<email-address>
        map(util::parse_nonempty_line("e="), |value| {
            Self(value.to_owned())
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let email_addr_str = "j.doe@example.com (Jane Doe)";
        let line = format!("e={}\nresto\r\n", email_addr_str);
        let (rest, email_addr) = EmailAddress::parse(&line).unwrap();
        assert_eq!(rest, "resto\r\n");
        assert_eq!(email_addr, EmailAddress(email_addr_str.to_owned()));
    }

    #[test]
    fn test_invalid() {
        assert!(EmailAddress::parse("e=\r\n").is_err());
        assert!(EmailAddress::parse("x=foo@bar.com\r\n").is_err());
    }
}
