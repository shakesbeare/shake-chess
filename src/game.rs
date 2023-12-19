use bevy::prelude::*;

#[derive(Component)]
pub struct Piece;

#[derive(Resource, Default)]
pub struct Board(pub chess::Board);

#[derive(Event, Debug)]
pub struct MoveEvent(pub Option<chess::ChessMove>);

pub fn setup_game(mut board: ResMut<Board>) {
    board.0 = chess::Board::default();
}

