pub mod client;
mod codec;
mod util;

type Codec = crate::codec::StunCodec<stun_codec::rfc5389::Attribute>;
type Message = stun_codec::Message<stun_codec::rfc5389::Attribute>;
