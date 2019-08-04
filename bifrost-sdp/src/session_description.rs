use nom::IResult;

use crate::{Information, Origin, Parse, SessionName, Url, Version};

// https://tools.ietf.org/html/rfc4566#section-5
#[derive(Debug, PartialEq)]
pub struct SessionDescription {
    pub version: Version,
    pub origin: Origin,
    pub session_name: SessionName,
    pub session_information: Option<Information>,
    pub url: Option<Url>,
}

impl Parse for SessionDescription {
    fn parse(input: &str) -> IResult<&str, SessionDescription> {
        let (rest, version) = Parse::parse(input)?;
        let (rest, origin) = Parse::parse(rest)?;
        let (rest, session_name) = Parse::parse(rest)?;
        let (rest, session_information) = Parse::parse(rest)?;
        let (rest, url) = Parse::parse(rest)?;

        Ok((
            rest,
            SessionDescription {
                version,
                origin,
                session_name,
                session_information,
                url,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s = r#"v=0
o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5
s=SDP Seminar
i=A Seminar on the session description protocol
u=http://www.example.com/seminars/sdp.pdf
"#;

        let expected = SessionDescription {
            version: Version {},
            origin: Origin {
                username: "jdoe".to_owned(),
                session_id: 2890844526,
                session_version: 2890842807,
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                unicast_address: "10.47.16.5".to_owned(),
            },
            session_name: SessionName("SDP Seminar".to_owned()),
            session_information: Some(Information(
                "A Seminar on the session description protocol".to_owned(),
            )),
            url: Some(Url::parse("http://www.example.com/seminars/sdp.pdf").unwrap()),
        };

        let (_, sdp) = SessionDescription::parse(s).unwrap();
        assert_eq!(sdp, expected);
    }
}
