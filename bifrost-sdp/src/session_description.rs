use nom::IResult;

use crate::{Information, Origin, Parse, SessionName, Version};

// https://tools.ietf.org/html/rfc4566#section-5
#[derive(Debug, PartialEq)]
pub struct SessionDescription {
    pub version: Version,
    pub origin: Origin,
    pub session_name: SessionName,
    pub session_information: Option<Information>,
}

impl Parse for SessionDescription {
    fn parse(input: &str) -> IResult<&str, SessionDescription> {
        let (rest, version) = Parse::parse(input)?;
        let (rest, origin) = Parse::parse(rest)?;
        let (rest, session_name) = Parse::parse(rest)?;
        let (rest, session_information) = Parse::parse(rest)?;

        Ok((
            rest,
            SessionDescription {
                version,
                origin,
                session_name,
                session_information,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s = "v=0\r\n\
                 o=mozilla...THIS_IS_SDPARTA-68.0 937286732060122712 0 IN IP4 0.0.0.0\r\n\
                 s=-\r\n";
        let expected = SessionDescription {
            version: Version {},
            origin: Origin {
                username: "mozilla...THIS_IS_SDPARTA-68.0".to_owned(),
                session_id: 937286732060122712,
                session_version: 0,
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                unicast_address: "0.0.0.0".to_owned(),
            },
            session_name: SessionName("-".to_owned()),
            session_information: None,
        };

        let (_, sdp) = SessionDescription::parse(s).unwrap();
        assert_eq!(sdp, expected);
    }
}
