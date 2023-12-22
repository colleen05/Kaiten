#[derive(Debug)]
pub enum Axis {
    Column,
    Row,
}

impl Axis {
    pub fn from_byte(byte: u8) -> Result<Axis, &'static str> {
        match byte {
            0x00 => Ok(Axis::Column),
            0x01 => Ok(Axis::Row),
            _ => Err("Axis byte must be either 0x00 (column) or 0x01 (row)."),
        }
    }

    pub fn as_byte(&self) -> u8 {
        match *self {
            Self::Column => 0x00,
            Self::Row => 0x01,
        }
    }
}
