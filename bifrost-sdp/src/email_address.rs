use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::IResult;

use crate::{util, Parse};

/// A parsed email address line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.6).
#[derive(Debug, PartialEq)]
pub struct EmailAddress<'a>(pub Cow<'a, str>);

impl<'a> Parse<'a> for EmailAddress<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // e=<email-address>
        let (rest, _) = tag("e=")(input)?;
        let (rest, value) = util::parse_line(rest)?;
        Ok((rest, Self(value)))
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
        assert_eq!(email_addr, EmailAddress(email_addr_str.into()));
    }

    #[test]
    fn test_invalid() {
        assert!(EmailAddress::parse("e=\r\n").is_err());
        assert!(EmailAddress::parse("x=foo@bar.com\r\n").is_err());
    }
}
