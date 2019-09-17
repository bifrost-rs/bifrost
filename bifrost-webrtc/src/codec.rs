use bifrost_stun::codec::MessageCodec;
use bifrost_stun::message::Message;
use bytes::BytesMut;
use std::io;
use tokio_codec::Decoder;

pub enum MuxMessage {
    Stun(Message),
    Unknown,
}

pub struct MuxDecoder {
    stun: MessageCodec,
}

impl Default for MuxDecoder {
    fn default() -> Self {
        Self {
            stun: MessageCodec::default(),
        }
    }
}

impl MuxDecoder {
    // TODO: Remove allow
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Decoder for MuxDecoder {
    type Item = MuxMessage;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.stun.decode(src) {
            Ok(Some(Some(item))) => return Ok(Some(MuxMessage::Stun(item))),
            Ok(Some(None)) => (),
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        }

        // TODO: Try next decoder

        Ok(Some(MuxMessage::Unknown))
    }
}
