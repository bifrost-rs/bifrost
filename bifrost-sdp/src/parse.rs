use nom::combinator::{map_res, opt};
use nom::multi::many1;
use nom::IResult;
use vec1::Vec1;

pub trait Parse: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;
}

impl<T: Parse> Parse for Option<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        opt(T::parse)(input)
    }
}

impl<T: Parse> Parse for Vec1<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(many1(T::parse), Self::try_from_vec)(input)
    }
}
