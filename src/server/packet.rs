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
            Packet::Empty => 0x00,
            Packet::Busy => 0x01,
            Packet::Info(_) => 0x02,
            Packet::UnknownCommand => 0x03,
            Packet::IllegalCommand => 0x04,
            Packet::JoinDenied => 0x05,
            Packet::JoinAccepted => 0x06,
            Packet::Kick => 0x07,
            Packet::GameBegin => 0x08,
            Packet::GameEnd => 0x09,
            Packet::TurnBegin => 0x0a,
            Packet::TurnEnd => 0x0b,
            Packet::UnknownAction => 0x0c,
            Packet::IllegalAction => 0x0d,
            Packet::Move(_) => 0x0e,
            Packet::Message(_) => 0x0f,
            Packet::UnknownError(_) => 0x10,
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
                let mv_bytes: [u8; 7] = bytes[1..8].try_into().unwrap(); // This should never fail, as the array is always 128 bytes.
                Ok(Packet::Move(Move::from_bytes(mv_bytes)?)) // Propogate any errors from Move deserialisation.
            }
            0x02 | 0x0f | 0x10 => {
                // Split at first 0x00 for null-terminated string.
                let mut message_split = bytes[1..].split(|b| *b == 0);

                // If a split could be made, decode the first half as UTF-8, otherwise just use an empty string.
                let message_string = match message_split.nth(0) {
                    Some(message_bytes) => {
                        match String::from_utf8(Vec::<u8>::from(message_bytes)) {
                            Ok(message) => message,
                            Err(_) => return Err("invalid utf-8 string."),
                        }
                    }
                    None => String::new(),
                };

                // Return the appropriate variant containing the message string.
                Ok(match type_byte {
                    0x02 => Packet::Info(message_string),
                    0x0f => Packet::Message(message_string),
                    0x10 => Packet::UnknownError(message_string),
                    _ => unreachable!("type byte already matched by parent arm."),
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
            // All message-containing variants can be serialised in the same way.
            Packet::Info(message) | Packet::Message(message) | Packet::UnknownError(message) => {
                let bytes = message.as_bytes();

                if bytes.len() > 126 {
                    return Err(String::from(
                        "string exceeded length limit of 126 characters.",
                    ));
                }

                variant_bytes.extend_from_slice(message.as_bytes());
                variant_bytes.push(0x00); // Make sure string ends in null terminator
            }
            // The Move varriant must be specially serialised.
            Packet::Move(mv) => variant_bytes.extend_from_slice(&mv.as_bytes()?),
            // Other variants have no additional data.
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
