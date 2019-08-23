use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

use crate::Parse;

#[derive(Clone, Debug, PartialEq)]
pub struct Instant(u64);

impl Instant {
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

    pub fn as_secs(&self) -> u64 {
        self.0
    }
}

impl Parse for Instant {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Duration(i64);

impl Duration {
    pub fn from_secs(secs: i64) -> Self {
        Self(secs)
    }

    pub fn from_mins(mins: i64) -> Self {
        Self(mins * 60)
    }

    pub fn from_hours(hours: i64) -> Self {
        Self(hours * 3600)
    }

    pub fn from_days(days: i64) -> Self {
        Self(days * 86400)
    }

    pub fn as_secs(&self) -> i64 {
        self.0
    }
}

impl Parse for Duration {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, sign) = match input.chars().next() {
            Some('+') => (&input[1..], true),
            Some('-') => (&input[1..], false),
            _ => (input, true),
        };

        let (rest, unsigned_time) = map_res(digit1, str::parse::<i64>)(rest)?;
        let time = if sign { unsigned_time } else { -unsigned_time };

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
    use crate::test_util::assert_err;

    #[test]
    fn test_valid_instants() {
        assert_eq!(Instant::from_days(42).as_secs(), 42 * 86400);
        assert_eq!(Instant::from_hours(41).as_secs(), 41 * 3600);
        assert_eq!(Instant::from_mins(40).as_secs(), 40 * 60);
        assert_eq!(Instant::from_secs(39).as_secs(), 39);

        assert_eq!(Instant::parse("42dx"), Ok(("x", Instant::from_days(42))));
        assert_eq!(Instant::parse("41h "), Ok((" ", Instant::from_hours(41))));
        assert_eq!(
            Instant::parse("40m 41h"),
            Ok((" 41h", Instant::from_mins(40)))
        );
        assert_eq!(
            Instant::parse("39s\r\n"),
            Ok(("\r\n", Instant::from_secs(39)))
        );
        assert_eq!(
            Instant::parse("38 37\r\n"),
            Ok((" 37\r\n", Instant::from_secs(38)))
        );
        assert_eq!(Instant::parse("37x"), Ok(("x", Instant::from_secs(37))));
    }

    #[test]
    fn test_invalid_instants() {
        assert_err::<Instant>("s");
        assert_err::<Instant>(" 42");
        assert_err::<Instant>("");
        assert_err::<Instant>(" ");
    }

    #[test]
    fn test_valid_durations() {
        assert_eq!(Duration::from_days(42).as_secs(), 42 * 86400);
        assert_eq!(Duration::from_hours(-41).as_secs(), -41 * 3600);
        assert_eq!(Duration::from_mins(40).as_secs(), 40 * 60);
        assert_eq!(Duration::from_secs(-39).as_secs(), -39);

        assert_eq!(Duration::parse("+42dx"), Ok(("x", Duration::from_days(42))));
        assert_eq!(
            Duration::parse("-41h "),
            Ok((" ", Duration::from_hours(-41)))
        );
        assert_eq!(
            Duration::parse("40m 41h"),
            Ok((" 41h", Duration::from_mins(40)))
        );
        assert_eq!(
            Duration::parse("-39s\r\n"),
            Ok(("\r\n", Duration::from_secs(-39)))
        );
        assert_eq!(
            Duration::parse("+38 37\r\n"),
            Ok((" 37\r\n", Duration::from_secs(38)))
        );
        assert_eq!(Duration::parse("-37x"), Ok(("x", Duration::from_secs(-37))));
    }

    #[test]
    fn test_invalid_durations() {
        assert_err::<Duration>("*42");
        assert_err::<Duration>(" 42");
        assert_err::<Duration>("s");
        assert_err::<Duration>("");
        assert_err::<Duration>(" ");
    }
}
