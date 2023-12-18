use kaiten::{server::Packet, Axis, Move, PlayerColour};

fn main() {
    let _p_move = Packet::Move(Move {
        player: PlayerColour::Black,
        axis: Axis::Row,
        position: 3,
        reverse: true,
        capture: Some((7, 9)),
    });

    let _p_message = Packet::Info(String::from("aaaaaaaaaaaaaaaaaaaaaaaa aaaaaaaaaaaaaaaaaaaaaaaa aaaaaaaaaaaaaaaaaaaaaaaa aaaaaaaaaaaaaaaaaaaaaaaa aaaaaaaaaaaaaaaaaaaaaaaa b"));

    // println!("{:?}:\n{:?}", _p_move, _p_move.as_bytes());
    println!("{:?}:\n{:?}", _p_message, _p_message.as_bytes());
}
