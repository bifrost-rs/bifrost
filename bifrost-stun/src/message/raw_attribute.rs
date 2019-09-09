use std::io;

#[derive(Debug)]
pub struct RawAttribute {
    pub r#type: u16,
    value: Vec<u8>,
}

impl RawAttribute {
    /// The maximum allowed length of an attribute in bytes. Set arbitrarily to
    /// 1024.
    pub const MAX_LEN: u16 = 1024;

    pub fn new(r#type: u16, value: Vec<u8>) -> io::Result<Self> {
        if value.len() <= Self::MAX_LEN as usize {
            Ok(Self { r#type, value })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "attribute length too large",
            ))
        }
    }

    pub fn unpadded_len(&self) -> u16 {
        self.value.len() as u16
    }

    pub fn padded_len(&self) -> u16 {
        (self.unpadded_len() + 3) & !0b11
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_len() {
        let a0 = RawAttribute::new(0, vec![]).unwrap();
        assert_eq!(a0.unpadded_len(), 0);
        assert_eq!(a0.padded_len(), 0);

        let a1 = RawAttribute::new(0, vec![0; 21]).unwrap();
        assert_eq!(a1.unpadded_len(), 21);
        assert_eq!(a1.padded_len(), 24);

        let a2 = RawAttribute::new(0, vec![0; RawAttribute::MAX_LEN as usize]).unwrap();
        assert_eq!(a2.unpadded_len(), RawAttribute::MAX_LEN);
        assert_eq!(a2.padded_len(), RawAttribute::MAX_LEN);
    }

    #[test]
    fn test_invalid_len() {
        assert!(RawAttribute::new(0, vec![0; RawAttribute::MAX_LEN as usize + 1]).is_err());
    }
}
