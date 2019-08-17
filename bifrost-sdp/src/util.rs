use std::borrow::Cow;
use std::str::FromStr;

use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::IResult;

pub fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
    let (rest, value) = is_not("\r\n")(input)?;
    let (rest, _) = line_ending(rest)?;
    Ok((rest, value.into()))
}

pub fn parse_str_field<'a>(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
    let (rest, value) = is_not(" \r\n")(input)?;
    Ok((rest, value.into()))
}

pub fn parse_field<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(is_not(" \r\n"), FromStr::from_str)(input)
}
