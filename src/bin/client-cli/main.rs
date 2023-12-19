use kaiten::{client::Packet, Axis, Move, PlayerColour};

fn main() {
    let _p_move = Packet::Move(Move {
        player: PlayerColour::Red,
        axis: Axis::Row,
        position: 3,
        reverse: true,
        capture: Some((7, 9)),
    });
    let _p_move_bytes = _p_move.as_bytes().unwrap();
    let _p_move_2 = Packet::from_bytes(_p_move_bytes).unwrap();

    let _p_message = Packet::Message(String::from("I am a cool client!"));
    let _p_message_bytes = _p_message.as_bytes().unwrap();
    let _p_message_2 = Packet::from_bytes(_p_message_bytes).unwrap();

    println!("{:?}:\n{:?}\n{:?}\n", _p_move, _p_move_2, _p_move_bytes);
    println!(
        "{:?}:\n{:?}\n{:?}\n",
        _p_message, _p_message_2, _p_message_bytes
    );
}
