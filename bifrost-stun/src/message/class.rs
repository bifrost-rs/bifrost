#[derive(Debug, Eq, PartialEq)]
pub enum Class {
    Request,
    Indication,
    SuccessResponse,
    FailureResponse,
}

impl Class {
    pub fn from_low_2_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0b00 => Self::Request,
            0b01 => Self::Indication,
            0b10 => Self::SuccessResponse,
            0b11 => Self::FailureResponse,
            _ => unreachable!(),
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self {
            Self::Request => 0b00,
            Self::Indication => 0b01,
            Self::SuccessResponse => 0b10,
            Self::FailureResponse => 0b11,
        }
    }
}
