use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_nonempty_list;
use nom::IResult;
use vec1::Vec1;

use crate::{util, Parse};

/// A parsed media information line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.14).
#[derive(Debug, PartialEq)]
pub struct MediaInformation {
    pub media_type: String,
    pub port: String,
    pub proto: String,
    pub formats: Vec1<String>,
}

impl Parse for MediaInformation {
    fn parse(input: &str) -> IResult<&str, Self> {
        // m=<media> <port> <proto> <fmt> ...
        let (rest, _) = tag("m=")(input)?;

        let (rest, media_type) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, port) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, proto) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, formats) = separated_nonempty_list(tag(" "), util::parse_field)(rest)?;
        let formats = Vec1::try_from_vec(formats).unwrap();
        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Self {
                media_type,
                port,
                proto,
                formats,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use vec1::vec1;

    use super::*;

    #[test]
    fn test_valid() {
        let s1 = "m=audio 49170 RTP/AVP 0\r\nmore\n";
        assert_eq!(
            MediaInformation::parse(s1),
            Ok((
                "more\n",
                MediaInformation {
                    media_type: "audio".to_owned(),
                    port: "49170".to_owned(),
                    proto: "RTP/AVP".to_owned(),
                    formats: vec1!["0".to_owned()],
                }
            ))
        );

        let s2 = "m=video 49170/2 RTP/AVP 31\r\nmore\n";
        assert_eq!(
            MediaInformation::parse(s2),
            Ok((
                "more\n",
                MediaInformation {
                    media_type: "video".to_owned(),
                    port: "49170/2".to_owned(),
                    proto: "RTP/AVP".to_owned(),
                    formats: vec1!["31".to_owned()],
                }
            ))
        );

        let s3 = "m=foo 12345 bar x y z\r\nmore\r\n";
        assert_eq!(
            MediaInformation::parse(s3),
            Ok((
                "more\r\n",
                MediaInformation {
                    media_type: "foo".to_owned(),
                    port: "12345".to_owned(),
                    proto: "bar".to_owned(),
                    formats: vec1!["x".to_owned(), "y".to_owned(), "z".to_owned()],
                }
            ))
        );
    }

    #[test]
    fn test_invalid() {
        assert!(MediaInformation::parse("n=audio 49170 RTP/AVP 0\r\nmore\n").is_err());
        assert!(MediaInformation::parse("m=audio 49170 RTP/AVP\r\nmore\n").is_err());
    }
}
