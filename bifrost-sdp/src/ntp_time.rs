use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

use crate::Parse;

#[derive(Debug, PartialEq)]
pub struct NtpTime(u64);

impl NtpTime {
    pub fn from_secs(secs: u64) -> Self {
        Self(secs)
    }

    pub fn from_mins(mins: u64) -> Self {
        Self(mins * 60)
    }

    pub fn from_hours(hours: u64) -> Self {
        Self(hours * 3600)
    }

    pub fn from_days(days: u64) -> Self {
        Self(days * 86400)
    }

    pub fn to_secs(&self) -> u64 {
        self.0
    }
}

impl Parse for NtpTime {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, time) = map_res(digit1, str::parse)(input)?;

        Ok(match rest.chars().next() {
            Some('d') => (&rest[1..], Self::from_days(time)),
            Some('h') => (&rest[1..], Self::from_hours(time)),
            Some('m') => (&rest[1..], Self::from_mins(time)),
            Some('s') => (&rest[1..], Self::from_secs(time)),
            _ => (rest, Self::from_secs(time)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units() {
        assert_eq!(NtpTime::from_days(42).to_secs(), 42 * 86400);
        assert_eq!(NtpTime::from_hours(41).to_secs(), 41 * 3600);
        assert_eq!(NtpTime::from_mins(40).to_secs(), 40 * 60);
        assert_eq!(NtpTime::from_secs(39).to_secs(), 39);
    }

    #[test]
    fn test_valid() {
        assert_eq!(NtpTime::parse("42dx"), Ok(("x", NtpTime::from_days(42))));
        assert_eq!(NtpTime::parse("41h "), Ok((" ", NtpTime::from_hours(41))));
        assert_eq!(
            NtpTime::parse("40m 41h"),
            Ok((" 41h", NtpTime::from_mins(40)))
        );
        assert_eq!(
            NtpTime::parse("39s\r\n"),
            Ok(("\r\n", NtpTime::from_secs(39)))
        );
        assert_eq!(
            NtpTime::parse("38 37\r\n"),
            Ok((" 37\r\n", NtpTime::from_secs(38)))
        );
        assert_eq!(NtpTime::parse("37x"), Ok(("x", NtpTime::from_secs(37))));
    }

    #[test]
    fn test_invalid() {
        assert!(NtpTime::parse("s").is_err());
        assert!(NtpTime::parse(" 42").is_err());
        assert!(NtpTime::parse("").is_err());
        assert!(NtpTime::parse(" ").is_err());
    }
}
