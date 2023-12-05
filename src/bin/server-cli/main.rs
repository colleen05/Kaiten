use kaiten::{server::Packet, Axis, Move, PlayerColour};

fn main() {
    let p = Packet::Move(Move {
        player: PlayerColour::Red,
        axis: Axis::Column,
        position: 0,
        reverse: false,
        capture: None,
    });

    println!("{:?}:\n{:?}", p, p.as_bytes());
}
