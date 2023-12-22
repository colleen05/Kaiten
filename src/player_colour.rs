#[derive(Debug)]
pub enum PlayerColour {
    Red,
    Black,
}

impl PlayerColour {
    pub fn from_byte(byte: u8) -> Result<PlayerColour, &'static str> {
        match byte {
            0x00 => Ok(PlayerColour::Red),
            0x01 => Ok(PlayerColour::Black),
            _ => Err("PlayerColour byte must be either 0x00 (red) or 0x01 (black)."),
        }
    }

    pub fn as_byte(&self) -> u8 {
        match *self {
            Self::Red => 0x00,
            Self::Black => 0x01,
        }
    }
}
