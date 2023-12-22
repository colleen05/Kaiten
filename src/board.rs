use crate::PlayerColour;

#[derive(Debug)]
pub struct Board {
    pub pieces: Vec<Option<PlayerColour>>,
}
