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
