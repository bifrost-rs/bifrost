#[derive(Debug)]
pub struct RawAttribute {
    pub r#type: u16,
    pub value: Vec<u8>,
}

impl RawAttribute {
    /// The largest `u16` value that is a multiple of 4.
    pub const MAX_LEN: u16 = u16::max_value() >> 2 << 2;
}
