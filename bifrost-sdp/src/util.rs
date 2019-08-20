use std::str::FromStr;

use nom::bytes::complete::{is_not, tag};
use nom::character::complete::line_ending;
use nom::combinator::{map, map_res};
use nom::IResult;

/// Parses the input until reaching a whitespace or a newline.
pub fn parse_raw_field<'a, T: From<&'a str>>(input: &'a str) -> IResult<&str, T> {
    map(is_not(" \r\n"), Into::into)(input)
}

pub fn parse_nonempty_line(type_tag: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
    move |input| {
        let (rest, _) = tag(type_tag)(input)?;
        let (rest, value) = is_not("\r\n")(rest)?;
        let (rest, _) = line_ending(rest)?;
        Ok((rest, value))
    }
}

pub fn parse_field<T: FromStr>(input: &str) -> IResult<&str, T> {
    let (rest, value) = map_res(is_not(" \r\n"), FromStr::from_str)(input)?;
    let (rest, _) = tag(" ")(rest)?;
    Ok((rest, value))
}

pub fn parse_last_field<T: FromStr>(input: &str) -> IResult<&str, T> {
    let (rest, value) = map_res(is_not(" \r\n"), FromStr::from_str)(input)?;
    let (rest, _) = line_ending(rest)?;
    Ok((rest, value))
}
