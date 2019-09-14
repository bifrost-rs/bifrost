use crate::{
    Attribute, Bandwidth, ConnectionData, EmailAddress, EncryptionKey, Information,
    MediaDescription, Origin, Parse, PhoneNumber, SessionName, TimeDescription, TimeZones, Uri,
    Version,
};
use nom::IResult;
use std::fmt;
use std::str::FromStr;
use vec1::Vec1;

/// A parsed SDP session description, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5).
#[derive(Clone, Debug, PartialEq)]
pub struct SessionDescription {
    pub version: Version,
    pub origin: Origin,
    pub session_name: SessionName,
    pub session_information: Option<Information>,
    pub uri: Option<Uri>,
    pub email_address: Option<EmailAddress>,
    pub phone_number: Option<PhoneNumber>,
    pub connection_data: Option<ConnectionData>,
    pub bandwidth: Option<Bandwidth>,
    pub time_descriptions: Vec1<TimeDescription>,
    pub time_zones: Option<TimeZones>,
    pub encryption_key: Option<EncryptionKey>,
    pub attributes: Vec<Attribute>,
    pub media_descriptions: Vec<MediaDescription>,
}

impl fmt::Display for SessionDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.version.fmt(f)?;
        self.origin.fmt(f)?;
        self.session_name.fmt(f)?;
        self.session_information.iter().try_for_each(|x| x.fmt(f))?;
        self.uri.iter().try_for_each(|x| x.fmt(f))?;
        self.email_address.iter().try_for_each(|x| x.fmt(f))?;
        self.phone_number.iter().try_for_each(|x| x.fmt(f))?;
        self.connection_data.iter().try_for_each(|x| x.fmt(f))?;
        self.bandwidth.iter().try_for_each(|x| x.fmt(f))?;
        self.time_descriptions.iter().try_for_each(|x| x.fmt(f))?;
        self.time_zones.iter().try_for_each(|x| x.fmt(f))?;
        self.encryption_key.iter().try_for_each(|x| x.fmt(f))?;
        self.attributes.iter().try_for_each(|x| x.fmt(f))?;
        self.media_descriptions.iter().try_for_each(|x| x.fmt(f))
    }
}

impl Parse for SessionDescription {
    fn parse(input: &str) -> IResult<&str, Self> {
        // v=  (protocol version)
        // o=  (originator and session identifier)
        // s=  (session name)
        // i=* (session information)
        // u=* (URI of description)
        // e=* (email address)
        // p=* (phone number)
        // c=* (connection information -- not required if included in all media)
        // b=* (zero or more bandwidth information lines)
        // One or more time descriptions ("t=" and "r=" lines; see below)
        // z=* (time zone adjustments)
        // k=* (encryption key)
        // a=* (zero or more session attribute lines)
        // Zero or more media descriptions

        let (rest, version) = Parse::parse(input)?;
        let (rest, origin) = Parse::parse(rest)?;
        let (rest, session_name) = Parse::parse(rest)?;
        let (rest, session_information) = Parse::parse(rest)?;
        let (rest, uri) = Parse::parse(rest)?;
        let (rest, email_address) = Parse::parse(rest)?;
        let (rest, phone_number) = Parse::parse(rest)?;
        let (rest, connection_data) = Parse::parse(rest)?;
        let (rest, bandwidth) = Parse::parse(rest)?;
        let (rest, time_descriptions) = Parse::parse(rest)?;
        let (rest, time_zones) = Parse::parse(rest)?;
        let (rest, encryption_key) = Parse::parse(rest)?;
        let (rest, attributes) = Parse::parse(rest)?;
        let (rest, media_descriptions) = Parse::parse(rest)?;

        Ok((
            rest,
            Self {
                version,
                origin,
                session_name,
                session_information,
                uri,
                email_address,
                phone_number,
                connection_data,
                bandwidth,
                time_descriptions,
                time_zones,
                encryption_key,
                attributes,
                media_descriptions,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseSdpError;

impl FromStr for SessionDescription {
    type Err = ParseSdpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
            .map_err(|_| ParseSdpError)
            .and_then(|(rest, value)| {
                if rest.is_empty() {
                    Ok(value)
                } else {
                    Err(ParseSdpError)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::assert_parse_display;
    use crate::{Duration, Instant, MediaInformation, RepeatTimes, TimeZone, Timing};
    use lazy_static::lazy_static;
    use vec1::vec1;

    const EXAMPLE_SDP_INPUT: &str = r#"v=0
o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5
s=SDP Seminar
i=A Seminar on the session description protocol
u=http://www.example.com/seminars/sdp.pdf
e=j.doe@example.com (Jane Doe)
c=IN IP4 224.2.36.42/127
b=X-YZ:128
t=3034423618 3042462418
t=3034423619 3042462419
r=604800 3600 0 90000
z=2882844526 -1h 2898848070 0
a=recvonly
m=audio 49170 RTP/AVP 0
m=video 51372 RTP/AVP 99
a=rtpmap:99 h263-1998/90000
"#;

    const EXAMPLE_SDP_OUTPUT: &str = "v=0\r\n\
                                      o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\r\n\
                                      s=SDP Seminar\r\n\
                                      i=A Seminar on the session description protocol\r\n\
                                      u=http://www.example.com/seminars/sdp.pdf\r\n\
                                      e=j.doe@example.com (Jane Doe)\r\n\
                                      c=IN IP4 224.2.36.42/127\r\n\
                                      b=X-YZ:128\r\n\
                                      t=3034423618 3042462418\r\n\
                                      t=3034423619 3042462419\r\n\
                                      r=604800 3600 0 90000\r\n\
                                      z=2882844526 -3600 2898848070 0\r\n\
                                      a=recvonly\r\n\
                                      m=audio 49170 RTP/AVP 0\r\n\
                                      m=video 51372 RTP/AVP 99\r\n\
                                      a=rtpmap:99 h263-1998/90000\r\n";

    lazy_static! {
        static ref EXAMPLE_SDP: SessionDescription = SessionDescription {
            version: Version,
            origin: Origin {
                username: "jdoe".to_owned(),
                session_id: 2_890_844_526,
                session_version: 2_890_842_807,
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                unicast_address: "10.47.16.5".to_owned(),
            },
            session_name: SessionName("SDP Seminar".to_owned()),
            session_information: Some(Information(
                "A Seminar on the session description protocol".to_owned(),
            )),
            uri: Some(Uri("http://www.example.com/seminars/sdp.pdf"
                .parse()
                .unwrap())),
            email_address: Some(EmailAddress("j.doe@example.com (Jane Doe)".to_owned())),
            phone_number: None,
            connection_data: Some(ConnectionData {
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                connection_address: "224.2.36.42/127".to_owned(),
            }),
            bandwidth: Some(Bandwidth {
                experimental: true,
                bwtype: "YZ".to_owned(),
                bandwidth: 128,
            }),
            time_descriptions: vec1![
                TimeDescription {
                    timing: Timing {
                        start_time: Instant::from_secs(3_034_423_618),
                        stop_time: Instant::from_secs(3_042_462_418),
                    },
                    repeat_times: vec![],
                },
                TimeDescription {
                    timing: Timing {
                        start_time: Instant::from_secs(3_034_423_619),
                        stop_time: Instant::from_secs(3_042_462_419),
                    },
                    repeat_times: vec![RepeatTimes {
                        interval: Duration::from_secs(604_800),
                        duration: Duration::from_secs(3600),
                        offsets: vec1![Duration::from_secs(0), Duration::from_secs(90000)],
                    }],
                }
            ],
            time_zones: Some(TimeZones(vec1![
                TimeZone {
                    adjustment_time: Instant::from_secs(2_882_844_526),
                    offset: Duration::from_hours(-1),
                },
                TimeZone {
                    adjustment_time: Instant::from_secs(2_898_848_070),
                    offset: Duration::from_secs(0),
                }
            ])),
            encryption_key: None,
            attributes: vec![Attribute {
                name: "recvonly".to_owned(),
                value: None,
            }],
            media_descriptions: vec![
                MediaDescription {
                    media_information: MediaInformation {
                        media_type: "audio".to_owned(),
                        port: "49170".to_owned(),
                        proto: "RTP/AVP".to_owned(),
                        formats: vec1!["0".to_owned()],
                    },
                    media_title: None,
                    connection_data: None,
                    bandwidths: vec![],
                    encryption_key: None,
                    attributes: vec![],
                },
                MediaDescription {
                    media_information: MediaInformation {
                        media_type: "video".to_owned(),
                        port: "51372".to_owned(),
                        proto: "RTP/AVP".to_owned(),
                        formats: vec1!["99".to_owned()],
                    },
                    media_title: None,
                    connection_data: None,
                    bandwidths: vec![],
                    encryption_key: None,
                    attributes: vec![Attribute {
                        name: "rtpmap".to_owned(),
                        value: Some("99 h263-1998/90000".to_owned()),
                    }],
                },
            ],
        };
    }

    #[test]
    fn valid() {
        assert_parse_display(
            &format!("{}more", EXAMPLE_SDP_INPUT),
            "more",
            &*EXAMPLE_SDP,
            &EXAMPLE_SDP_OUTPUT,
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            EXAMPLE_SDP_INPUT.parse::<SessionDescription>(),
            Ok(EXAMPLE_SDP.clone())
        );

        let invalid_str_1 = "foo";
        assert_eq!(
            invalid_str_1.parse::<SessionDescription>(),
            Err(ParseSdpError)
        );

        let invalid_str_2 = format!("{}more", EXAMPLE_SDP_INPUT);
        assert_eq!(
            invalid_str_2.parse::<SessionDescription>(),
            Err(ParseSdpError)
        );
    }
}
