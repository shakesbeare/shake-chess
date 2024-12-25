use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy::winit::cursor::CursorIcon;
use bevy_svg::prelude::*;
use chess::{BitBoard, File, Rank, Square};

const SPRITE_SIZE: f32 = 45.;
const BOARD_LENGTH: i32 = 8;
const VERT_BOARD_PERCENT: f32 = 0.90; // the max proportion of the vertical space which the board takes up
const HORI_BOARD_PERCENT: f32 = 0.90; // ... horizontal

const LIGHT_SQUARE_COLOR: &str = "#F0D9B5"; // stolen from lichess
const DARK_SQUARE_COLOR: &str = "#B58863"; // ... again
pub const BACKGROUND_COLOR: &str = "#313338"; // stolen from discord

#[allow(dead_code)]
#[derive(Resource, Default, Debug)]
pub struct DrawInfo {
    square_size: f32,
}

pub fn update_draw_info(
    window: Query<&Window>,
    mut window_ev: EventReader<WindowResized>,
    mut draw_info: ResMut<DrawInfo>,
) {
    if window_ev.is_empty() {
        return;
    }

    for ev in window_ev.read() {
        debug!("{ev:?}");
    }

    let window = window.single();
    let width = window.width();
    let height = window.height();

    let square_size = {
        let board_vertical_space = height * VERT_BOARD_PERCENT;
        let board_horizontal_space = width * HORI_BOARD_PERCENT;

        if board_vertical_space > board_horizontal_space {
            board_horizontal_space / BOARD_LENGTH as f32
        } else {
            (height * VERT_BOARD_PERCENT) / BOARD_LENGTH as f32
        }
    };

    *draw_info = DrawInfo { square_size };
}

pub fn draw_chessboard(
    draw_info: Res<DrawInfo>,
    entities: Query<Entity, With<crate::Square>>,
    mut commands: Commands,
    mut window_ev: EventReader<WindowResized>,
) {
    if window_ev.is_empty() {
        return;
    }
    for _ in window_ev.read() {}
    debug!("Redrawing board...");

    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }

    let offset = -draw_info.square_size * BOARD_LENGTH as f32 / 2.;
    commands.spawn((
        Sprite {
            color: Srgba::hex(LIGHT_SQUARE_COLOR).unwrap().into(),
            custom_size: Some(Vec2::new(
                draw_info.square_size * BOARD_LENGTH as f32,
                draw_info.square_size * BOARD_LENGTH as f32,
            )),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        crate::Square,
    ));

    for i in 0..BOARD_LENGTH {
        for j in 0..BOARD_LENGTH {
            if (i + j) % 2 == 0 {
                continue;
            }
            commands.spawn((
                Sprite {
                    color: Srgba::hex(DARK_SQUARE_COLOR).unwrap().into(),
                    custom_size: Some(Vec2::new(draw_info.square_size, draw_info.square_size)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    offset + (i as f32) * draw_info.square_size + (draw_info.square_size / 2.),
                    offset + (j as f32) * draw_info.square_size + (draw_info.square_size / 2.),
                    1.0,
                )),
                crate::Square,
            ));
        }
    }
}

pub fn draw_pieces(
    mut commands: Commands,
    board: Res<crate::game::Board>,
    draw_info: Res<DrawInfo>,
    entities: Query<Entity, With<crate::Piece>>,
    asset_server: Res<AssetServer>,
    mut move_ev: EventReader<crate::game::MoveEvent>,
    mut window_ev: EventReader<WindowResized>,
) {
    if move_ev.is_empty() && window_ev.is_empty() {
        return;
    }

    for _ in move_ev.read() {}
    for _ in window_ev.read() {}
    debug!("Redrawing pieces...");

    // Every piece is despawned and recreated when a move occurs
    // This is because we have to synchronize the sprites with the `chess` representation
    //     of the pieces.
    // It's way easier this way

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let offset = -draw_info.square_size * BOARD_LENGTH as f32 / 2.;
    for square in board.0.combined().into_iter() {
        let color = board.0.color_on(square).unwrap();
        let piece = board.0.piece_on(square).unwrap();
        let rank = square.get_rank().to_index() as f32;
        let file = square.get_file().to_index() as f32;

        let filename = match (piece, color) {
            (chess::Piece::Pawn, chess::Color::White) => "white_pawn.svg",
            (chess::Piece::Pawn, chess::Color::Black) => "black_pawn.svg",
            (chess::Piece::Knight, chess::Color::White) => "white_knight.svg",
            (chess::Piece::Knight, chess::Color::Black) => "black_knight.svg",
            (chess::Piece::Bishop, chess::Color::White) => "white_bishop.svg",
            (chess::Piece::Bishop, chess::Color::Black) => "black_bishop.svg",
            (chess::Piece::Rook, chess::Color::White) => "white_rook.svg",
            (chess::Piece::Rook, chess::Color::Black) => "black_rook.svg",
            (chess::Piece::Queen, chess::Color::White) => "white_queen.svg",
            (chess::Piece::Queen, chess::Color::Black) => "black_queen.svg",
            (chess::Piece::King, chess::Color::White) => "white_king.svg",
            (chess::Piece::King, chess::Color::Black) => "black_king.svg",
        };
        let svg = asset_server.load(filename);
        commands.spawn((
            Svg2d(svg.clone()),
            Origin::Center,
            Transform::from_translation(Vec3::new(
                offset + file * draw_info.square_size + (draw_info.square_size / 2.),
                offset + rank * draw_info.square_size + (draw_info.square_size / 2.),
                2.0,
            ))
            .with_scale(Vec3::new(
                draw_info.square_size / SPRITE_SIZE,
                draw_info.square_size / SPRITE_SIZE,
                1.0,
            )),
            crate::Piece,
        ));
    }
}

pub fn mouse_hover(
    mut cursor: Query<&mut CursorIcon>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    draw_info: Res<DrawInfo>,
    board: Res<crate::game::Board>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();
    let mut cursor = cursor.single_mut();

    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());

    let Some(pos) = cursor_pos else {
        return;
    };

    let board_bound = draw_info.square_size * 4.0;

    if pos.x < -board_bound || pos.x > board_bound || pos.y < -board_bound || pos.y > board_bound {
        *cursor = CursorIcon::System(bevy::window::SystemCursorIcon::Default);
        return;
    }

    let cur_square = Vec2::new(
        ((pos.x + (board_bound)) / draw_info.square_size).ceil() - 1.,
        ((pos.y + (board_bound)) / draw_info.square_size).ceil() - 1.,
    );

    let rank = Rank::from_index(cur_square.y as usize);
    let file = File::from_index(cur_square.x as usize);
    let square = Square::make_square(rank, file);
    let square_bb = BitBoard::from_square(square);
    let turn = board.0.side_to_move();
    let board_bb = board.0.color_combined(turn);
    let occupied = board_bb & square_bb;

    if occupied.0 > 0 {
        *cursor = CursorIcon::System(bevy::window::SystemCursorIcon::Grab);
    } else {
        *cursor = CursorIcon::System(bevy::window::SystemCursorIcon::Default);
    }
}
