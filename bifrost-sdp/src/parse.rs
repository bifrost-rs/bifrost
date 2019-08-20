use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::IResult;
use vec1::Vec1;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    InvalidFormat(nom::Err<(&'a str, nom::error::ErrorKind)>),
    TrailingCharacters(&'a str),
}

pub trait Parse: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;

    fn parse_all(input: &str) -> Result<Self, Error> {
        Self::parse(input)
            .map_err(Error::InvalidFormat)
            .and_then(|(rest, value)| {
                if rest.is_empty() {
                    Ok(value)
                } else {
                    Err(Error::TrailingCharacters(rest))
                }
            })
    }
}

impl<T: Parse> Parse for Option<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        opt(T::parse)(input)
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        many0(T::parse)(input)
    }
}

impl<T: Parse> Parse for Vec1<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, vec) = many1(T::parse)(input)?;
        Ok((rest, Self::try_from_vec(vec).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::map_res;

    use super::*;

    #[derive(Debug, PartialEq)]
    struct Test(i64);

    impl Parse for Test {
        fn parse(input: &str) -> IResult<&str, Self> {
            let (rest, x) = map_res(digit1, str::parse)(input)?;
            let (rest, _) = tag(".")(rest)?;

            Ok((rest, Self(x)))
        }
    }

    #[test]
    fn test_basic() {
        assert_eq!(Test::parse("123.x"), Ok(("x", Test(123))));
        assert!(Test::parse("foo").is_err());
    }

    #[test]
    fn test_opt() {
        assert_eq!(
            <Option<Test> as Parse>::parse("123.x"),
            Ok(("x", Some(Test(123))))
        );
        assert_eq!(<Option<Test> as Parse>::parse("foo"), Ok(("foo", None)));
    }

    #[test]
    fn test_vec() {
        assert_eq!(<Vec<Test> as Parse>::parse("foo"), Ok(("foo", vec![])));
        assert_eq!(
            <Vec<Test> as Parse>::parse("123.foo"),
            Ok(("foo", vec![Test(123)]))
        );
        assert_eq!(
            <Vec<Test> as Parse>::parse("123.456.foo"),
            Ok(("foo", vec![Test(123), Test(456)]))
        );
    }

    #[test]
    fn test_vec1() {
        use vec1::vec1;

        assert!(<Vec1<Test> as Parse>::parse("foo").is_err());
        assert_eq!(
            <Vec1<Test> as Parse>::parse("123.foo"),
            Ok(("foo", vec1![Test(123)]))
        );
        assert_eq!(
            <Vec1<Test> as Parse>::parse("123.456.foo"),
            Ok(("foo", vec1![Test(123), Test(456)]))
        );
    }

    #[test]
    fn test_parse_all() {
        assert_eq!(Test::parse_all("123."), Ok(Test(123)));

        assert!(match Test::parse_all("123") {
            Err(Error::InvalidFormat(_)) => true,
            _ => false,
        });

        assert_eq!(
            Test::parse_all("123.x"),
            Err(Error::TrailingCharacters("x"))
        );
    }
}
