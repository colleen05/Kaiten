use crate::{Axis, PlayerColour};

#[derive(Debug)]
pub struct PlayerMove {
    pub player: PlayerColour,
    pub axis: Axis,
    pub position: i32,
    pub reverse: bool,
    pub capture: Option<(i32, i32)>,
}

#[allow(clippy::new_without_default)]
impl PlayerMove {
    pub fn new() -> PlayerMove {
        PlayerMove {
            player: PlayerColour::Red,
            axis: Axis::Column,
            position: 0,
            reverse: false,
            capture: None,
        }
    }

    pub fn from_bytes(bytes: [u8; 7]) -> Result<PlayerMove, &'static str> {
        let mut mv = PlayerMove::new();

        mv.player = PlayerColour::from_byte(bytes[0])?;
        mv.axis = Axis::from_byte(bytes[1])?;
        mv.position = bytes[2] as i32;
        mv.reverse = bytes[3] != 0;

        mv.capture = match bytes[4] {
            0x00 => None,
            _ => Some((bytes[5] as i32, bytes[6] as i32)),
        };

        Ok(mv)
    }

    pub fn as_bytes(&self) -> Result<[u8; 7], String> {
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
            Err(e) => Err(format!("could not convert PlayerMove to bytes: {}", e)),
        }
    }
}
