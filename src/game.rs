use bevy::{prelude::*, window::PrimaryWindow};
use chess::{File, Rank, Square, Piece};

use crate::render::DrawInfo;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Board(chess::Board);


#[derive(Event, Debug)]
pub struct MoveEvent(pub Option<chess::ChessMove>);

pub fn setup_game(mut board: ResMut<Board>) {
    board.0 = chess::Board::default();
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct PointedSquare(Option<chess::Square>);

impl PointedSquare {
    pub fn set(&mut self, square: Square) {
        self.0 = Some(square);
    }
}

#[derive(Resource)]
pub enum SelectedPiece {
    None,
    Some {
        piece: Piece,
        square: Square,
    },
}

pub fn mouse_point(
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    draw_info: Res<DrawInfo>,
    mut pointed_square: ResMut<PointedSquare>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();
    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());

    let Some(pos) = cursor_pos else {
        return;
    };

    let square_size = draw_info.get_square_size();
    let board_bound = square_size * 4.0;

    if pos.x < -board_bound || pos.x > board_bound || pos.y < -board_bound || pos.y > board_bound {
        pointed_square.0 = None;
        return;
    }

    let cur_square = Vec2::new(
        ((pos.x + (board_bound)) / square_size).ceil() - 1.,
        ((pos.y + (board_bound)) / square_size).ceil() - 1.,
    );

    let rank = Rank::from_index(cur_square.y as usize);
    let file = File::from_index(cur_square.x as usize);
    let square = Square::make_square(rank, file);
    pointed_square.set(square);
}

pub fn select_piece(pointed_square: Res<PointedSquare>, input: Res<ButtonInput<MouseButton>>, board: Res<Board>, mut selected_piece: ResMut<SelectedPiece>) {
    if input.pressed(MouseButton::Left) && pointed_square.is_some() {
        let square = pointed_square.unwrap();
        let turn = board.side_to_move();
        if board.color_on(square).is_some() && board.color_on(square).unwrap() != turn {
            return;
        }

        if let Some(piece) = board.piece_on(square) {
            *selected_piece = SelectedPiece::Some {
                piece,
                square,
            }
        }
    }
}
