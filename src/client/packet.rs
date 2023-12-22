use crate::PlayerMove;

#[derive(Debug)]
pub enum Packet {
    Empty,
    InfoRequest,
    JoinRequest,
    Leave,
    PlayerMove(PlayerMove),
    InvalidCommand,
    IllegalCommand,
    Message(String),
}

impl Packet {
    pub fn header_code(&self) -> u8 {
        match *self {
            Packet::Empty => 0x00,
            Packet::InfoRequest => 0x01,
            Packet::JoinRequest => 0x02,
            Packet::Leave => 0x03,
            Packet::PlayerMove(_) => 0x04,
            Packet::InvalidCommand => 0x05,
            Packet::IllegalCommand => 0x06,
            Packet::Message(_) => 0x07,
        }
    }

    pub fn from_bytes(bytes: [u8; 128]) -> Result<Packet, &'static str> {
        let type_byte = bytes[0];

        match type_byte {
            0x00 => Ok(Packet::Empty),
            0x01 => Ok(Packet::InfoRequest),
            0x02 => Ok(Packet::JoinRequest),
            0x03 => Ok(Packet::Leave),
            0x04 => {
                let mv_bytes: [u8; 7] = bytes[1..8].try_into().unwrap(); // This should never fail, as the array is always 128 bytes.
                Ok(Packet::PlayerMove(PlayerMove::from_bytes(mv_bytes)?)) // Propogate any errors from Move deserialisation.
            }
            0x05 => Ok(Packet::InvalidCommand),
            0x06 => Ok(Packet::IllegalCommand),
            0x07 => {
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
                Ok(Packet::Message(message_string))
            }
            _ => Err(
                "first byte indicated an unknown packet type (byte must be in range 0x00..=0x06).",
            ),
        }
    }

    pub fn to_bytes(&self) -> Result<[u8; 128], String> {
        // Initialise byte arrays for final packet data and enum variant data.
        let mut packet_bytes = [0xffu8; 128];
        let mut variant_bytes = Vec::<u8>::with_capacity(127);

        // Set first byte of packet to variant type.
        packet_bytes[0] = self.header_code();

        // Get variant data.
        match self {
            Packet::PlayerMove(mv) => variant_bytes.extend_from_slice(&mv.to_bytes()?),
            Packet::Message(message) => {
                let bytes = message.as_bytes();

                if bytes.len() > 126 {
                    return Err(String::from(
                        "string exceeded length limit of 126 characters.",
                    ));
                }

                variant_bytes.extend_from_slice(message.as_bytes());
                variant_bytes.push(0x00); // Make sure string ends in null terminator
            }
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
