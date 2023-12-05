pub mod client;
pub mod server;

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

    pub fn as_byte(self) -> u8 {
        match self {
            Self::Red => 0x00,
            Self::Black => 0x01,
        }
    }
}

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

    pub fn as_byte(self) -> u8 {
        match self {
            Self::Column => 0x00,
            Self::Row => 0x01,
        }
    }
}

pub struct Move {
    player: PlayerColour,
    axis: Axis,
    position: i32,
    reverse: bool,
    capture: Option<(i32, i32)>,
}

#[allow(clippy::new_without_default)]
impl Move {
    pub fn new() -> Move {
        Move {
            player: PlayerColour::Black,
            axis: Axis::Column,
            position: 0,
            reverse: false,
            capture: Some((0, 0)),
        }
    }

    pub fn from_bytes(bytes: [u8; 7]) -> Result<Move, &'static str> {
        let mut mv = Move::new();

        let player = PlayerColour::from_byte(bytes[0]);
        match player {
            Ok(v) => mv.player = v,
            Err(e) => return Err(e),
        }

        let axis = Axis::from_byte(bytes[1]);
        match axis {
            Ok(v) => mv.axis = v,
            Err(e) => return Err(e),
        }

        mv.position = bytes[2] as i32;
        mv.reverse = bytes[3] != 0;

        mv.capture = match bytes[4] {
            0x00 => None,
            _ => Some((bytes[5] as i32, bytes[6] as i32)),
        };

        Ok(mv)
    }

    pub fn as_bytes(self) -> Result<[u8; 7], String> {
        let mut bytes = Vec::<u8>::with_capacity(7);

        bytes.push(self.player.as_byte());
        bytes.push(self.axis.as_byte());
        bytes.push(self.position as u8);
        bytes.push(self.reverse as u8);

        match self.capture {
            None => bytes.extend([0x00; 3]),
            Some(p) => bytes.extend(vec![0x01, p.0 as u8, p.1 as u8]),
        }

        match bytes.as_slice().try_into() {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("Could not convert Move to bytes: {}", e)),
        }
    }
}

pub struct Board {
    pieces: Vec<Option<PlayerColour>>,
}
