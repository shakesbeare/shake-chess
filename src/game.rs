use bevy::{prelude::*, window::PrimaryWindow};
use chess::{ChessMove, File, Piece, Rank, Square};

use crate::{render::DrawInfo, GameState, SideToMove};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Board(chess::Board);

#[derive(Event, Debug)]
pub struct SparseUpdateEvent;

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

#[derive(Resource, Clone, Copy)]
pub enum SelectedPiece {
    None,
    Some { piece: Piece, square: Square },
}

pub fn mouse_point(
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    side_to_move: Res<crate::SideToMove>,
    switch_sides: Res<crate::SwitchSides>,
    draw_info: Res<DrawInfo>,
    mut pointed_square: ResMut<PointedSquare>,
) {
    let is_white = side_to_move.0 == chess::Color::White;
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

    let mut cur_square = Vec2::new(
        ((pos.x + (board_bound)) / square_size).ceil() - 1.,
        ((pos.y + (board_bound)) / square_size).ceil() - 1.,
    );

    if !is_white && switch_sides.0 {
        cur_square.y = 7. - cur_square.y;
    }

    let rank = Rank::from_index(cur_square.y as usize);
    let file = File::from_index(cur_square.x as usize);
    let square = Square::make_square(rank, file);
    pointed_square.set(square);
}

pub fn act(
    pointed_square: Res<PointedSquare>,
    input: Res<ButtonInput<MouseButton>>,
    mut side_to_move: ResMut<SideToMove>,
    mut board: ResMut<Board>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut move_writer: EventWriter<SparseUpdateEvent>,
) {
    if input.just_pressed(MouseButton::Left) && pointed_square.is_some() {
        let square = pointed_square.unwrap();
        let target_col = board.color_on(square);

        match (*selected_piece, target_col) {
            (SelectedPiece::None, None) => return,
            (SelectedPiece::None, Some(col)) => {
                try_select(col, square, board.as_mut(), selected_piece.as_mut());
            }
            (SelectedPiece::Some { square: source, .. }, None) => {
                make_move(
                    source,
                    square,
                    None,
                    board.as_mut(),
                    selected_piece.as_mut(),
                    side_to_move.as_mut(),
                );
            }
            (SelectedPiece::Some { square: source, .. }, Some(col)) => {
                if try_select(col, square, board.as_mut(), selected_piece.as_mut()) {
                } else {
                    make_move(
                        source,
                        square,
                        None,
                        board.as_mut(),
                        selected_piece.as_mut(),
                        side_to_move.as_mut(),
                    );
                }
            }
        }
        move_writer.send(SparseUpdateEvent);
    }
}

fn make_move(
    source: Square,
    dest: Square,
    promotion: Option<Piece>,
    board: &mut Board,
    selected_piece: &mut SelectedPiece,
    side_to_move: &mut SideToMove,
) {
    // TODO: promotion
    let m = ChessMove::new(source, dest, None);
    if board.legal(m) {
        **board = board.make_move_new(m);
        *selected_piece = SelectedPiece::None;
        side_to_move.0 = board.side_to_move();
    } else {
        *selected_piece = SelectedPiece::None;
    }
}

fn try_select(
    target_color: chess::Color,
    square: Square,
    board: &mut Board,
    selected_piece: &mut SelectedPiece,
) -> bool {
    if target_color == board.side_to_move() {
        *selected_piece = SelectedPiece::Some {
            square,
            piece: board.piece_on(square).unwrap(),
        };
        true
    } else {
        false
    }
}

pub fn check_end(
    mut up_ev: EventReader<SparseUpdateEvent>,
    board: ResMut<Board>,
    mut state: ResMut<NextState<GameState>>,
) {
    for _ in up_ev.read() {
        match board.status() {
            chess::BoardStatus::Ongoing => {}
            chess::BoardStatus::Stalemate => {
                info!("Stalemate");
            }
            chess::BoardStatus::Checkmate => {
                let color = !board.side_to_move();
                state.set(GameState::Checkmate { winner: color });
                info!("Checkmate! Winner: {:?}", color);
            }
        }
    }
}
