use crate::{Board, Move};

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
    pub fn header_code(self) -> u8 {
        match self {
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

    pub fn as_bytes(self) -> [u8; 128] {
        let mut bytes = [0x00u8; 128];
        bytes[0] = self.header_code();

        todo!("Implement Packet serialisation.");
    }
}
