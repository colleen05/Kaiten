use crate::Move;

#[derive(Debug)]
pub enum Packet {
    Empty,
    Busy,
    Info(String),
    UnknownCommand,
    IllegalCommand,
    JoinDenied,
    JoinAccepted,
    Kick,
    GameBegin,
    GameEnd,
    TurnBegin,
    TurnEnd,
    UnknownAction,
    IllegalAction,
    Move(Move),
    Message(String),
    UnknownError(String),
}

impl Packet {
    pub fn header_code(&self) -> u8 {
        match *self {
            Self::Empty => 0x00,
            Self::Busy => 0x01,
            Self::Info(_) => 0x02,
            Self::UnknownCommand => 0x03,
            Self::IllegalCommand => 0x04,
            Self::JoinDenied => 0x05,
            Self::JoinAccepted => 0x06,
            Self::Kick => 0x07,
            Self::GameBegin => 0x08,
            Self::GameEnd => 0x09,
            Self::TurnBegin => 0x0a,
            Self::TurnEnd => 0x0b,
            Self::UnknownAction => 0x0c,
            Self::IllegalAction => 0x0d,
            Self::Move(_) => 0x0e,
            Self::Message(_) => 0x0f,
            Self::UnknownError(_) => 0x10,
        }
    }

    pub fn from_bytes(bytes: [u8; 128]) -> Result<Packet, &'static str> {
        let type_byte = bytes[0];

        match type_byte {
            0x00 => Ok(Packet::Empty),
            0x01 => Ok(Packet::Busy),
            /* 0x02: Deferred Packet::Info deserialisation */
            0x03 => Ok(Packet::UnknownCommand),
            0x04 => Ok(Packet::IllegalCommand),
            0x05 => Ok(Packet::JoinDenied),
            0x06 => Ok(Packet::JoinAccepted),
            0x07 => Ok(Packet::Kick),
            0x08 => Ok(Packet::GameBegin),
            0x09 => Ok(Packet::GameEnd),
            0x0a => Ok(Packet::TurnBegin),
            0x0b => Ok(Packet::TurnEnd),
            0x0c => Ok(Packet::UnknownAction),
            0x0d => Ok(Packet::IllegalAction),
            0x0e => {
                let mv_bytes: [u8; 7] = bytes[1..8].try_into().unwrap();
                Ok(Packet::Move(Move::from_bytes(mv_bytes)?))
            }
            0x02 | 0x0f | 0x10 => {
                let mut message_split = bytes[1..].split(|b| *b == 0);
                let mut message_string = String::new();

                if let Some(message_bytes) = message_split.nth(0) {
                    match String::from_utf8(Vec::<u8>::from(message_bytes)) {
                        Ok(message) => message_string = message,
                        Err(_) => return Err("invalid utf-8 string."),
                    }
                }

                Ok(match type_byte {
                    0x02 => Packet::Info(message_string),
                    0x0f => Packet::Message(message_string),
                    0x10 => Packet::UnknownError(message_string),
                    _ => unreachable!(),
                })
            }
            _ => Err(
                "first byte indicated an unknown packet type (byte must be in range 0x00..=0x10).",
            ),
        }
    }

    pub fn as_bytes(&self) -> Result<[u8; 128], String> {
        // Initialise byte arrays for final packet data and enum variant data.
        let mut packet_bytes = [0xffu8; 128];
        let mut variant_bytes = Vec::<u8>::with_capacity(127);

        // Set first byte of packet to variant type.
        packet_bytes[0] = self.header_code();

        // Get variant data.
        match self {
            Packet::Info(message) | Packet::Message(message) | Packet::UnknownError(message) => {
                let bytes = message.as_bytes();

                if bytes.len() > 126 {
                    return Err(String::from(
                        "string exceeded length limit of 126 characters.",
                    ));
                }

                variant_bytes.extend_from_slice(message.as_bytes());
                variant_bytes.push(0x00);
            }
            Packet::Move(mv) => variant_bytes.extend_from_slice(&mv.as_bytes()?),
            _ => {}
        }

        assert!(
            variant_bytes.len() <= 127,
            "valid packet could not be generated: Packet variant serialisation exceeded length of 127 bytes."
        );

        // Fill remaining packet bytes with variant data.
        for (i, b) in variant_bytes.iter().enumerate() {
            packet_bytes[1 + i] = *b;
        }

        // Return packet data.
        Ok(packet_bytes)
    }
}
