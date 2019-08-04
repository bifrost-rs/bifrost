use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, not_line_ending};
use nom::combinator::verify;
use nom::IResult;

pub fn parse_nonempty_line(type_tag: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
    move |input| {
        let (rest, _) = tag(type_tag)(input)?;
        let (rest, value) = verify(not_line_ending, |s: &str| !s.is_empty())(rest)?;
        let (rest, _) = line_ending(rest)?;
        Ok((rest, value))
    }
}
