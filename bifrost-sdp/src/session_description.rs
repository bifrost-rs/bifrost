use nom::IResult;

use crate::{Origin, SessionName, Version};

#[derive(Debug, PartialEq)]
pub struct SessionDescription {
    pub version: Version,
    pub origin: Origin,
    pub session_name: SessionName,
}

impl SessionDescription {
    pub fn parse(input: &str) -> IResult<&str, SessionDescription> {
        let (input, version) = Version::parse(input)?;
        let (input, origin) = Origin::parse(input)?;
        let (input, session_name) = SessionName::parse(input)?;
        Ok((
            input,
            SessionDescription {
                version,
                origin,
                session_name,
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
        };

        let (_, sdp) = SessionDescription::parse(s).unwrap();
        assert_eq!(sdp, expected);
    }
}
