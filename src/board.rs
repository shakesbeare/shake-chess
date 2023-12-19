use crate::Textures;
use chess::Piece;
use macroquad::prelude::*;

const BOARD_LENGTH: i32 = 8;
const VERT_BOARD_PERCENT: f32 = 0.75; // the proportion of the vertical space which the board takes up
const HORI_BOARD_PERCENT: f32 = 0.50; // ... horizontal
//
const LIGHT_SQUARE_COLOR: u32 = 0xf0d9b5; // stolen from lichess
const DARK_SQUARE_COLOR: u32 = 0xb58863; // ... again
const BACKGROUND_COLOR: u32 = 0x313338; // stolen from discord

pub struct DrawInfo {
    square_size: f32,
    x_start: f32,
    y_start: f32,
}

/// Calculate the necessary dimensions for the chessboard
pub fn calculate_draw_info() -> DrawInfo {
    let width = screen_width();
    let height = screen_height();
    let (square_size, y_start, x_start) = {
        let board_horizontal_space = height * VERT_BOARD_PERCENT;
        let board_allowed_space = width * HORI_BOARD_PERCENT;

        if board_horizontal_space > board_allowed_space {
            (
                board_allowed_space / BOARD_LENGTH as f32,
                height * (1. - VERT_BOARD_PERCENT) / 2.,
                (width - (board_allowed_space)) / 2.,
            )
        } else {
            (
                (height * VERT_BOARD_PERCENT) / BOARD_LENGTH as f32,
                height * (1. - VERT_BOARD_PERCENT) / 2.,
                (width - (height * VERT_BOARD_PERCENT)) / 2.,
            )
        }
    };


    DrawInfo {
        square_size,
        x_start,
        y_start,
    }
}

pub fn draw_board(draw_info: &DrawInfo) {
    clear_background(Color::from_hex(BACKGROUND_COLOR));
    dbg!(&draw_info.square_size);

    draw_rectangle(
        draw_info.x_start,
        draw_info.y_start,
        draw_info.square_size * BOARD_LENGTH as f32,
        draw_info.square_size * BOARD_LENGTH as f32,
        Color::from_hex(LIGHT_SQUARE_COLOR),
    );

    for i in 0..BOARD_LENGTH {
        for j in 0..BOARD_LENGTH {
            if (i + j) % 2 == 0 {
                continue;
            }

            draw_rectangle(
                draw_info.x_start + (i as f32 * draw_info.square_size),
                draw_info.y_start + (j as f32 * draw_info.square_size),
                draw_info.square_size,
                draw_info.square_size,
                Color::from_hex(DARK_SQUARE_COLOR),
            )
        }
    }
}

pub fn draw_pieces(board: chess::Board, draw_info: &DrawInfo, textures: &Textures) {
    let draw_params = DrawTextureParams {
        dest_size: Some((draw_info.square_size, draw_info.square_size).into()),
        ..Default::default()
    };

    for square in board.pieces(Piece::Pawn).into_iter() {
        let texture = match board.color_on(square).unwrap() {
            chess::Color::White => &textures.white_pawn,
            chess::Color::Black => &textures.black_pawn,
        };
        let rank = square.get_rank().to_index() as f32;
        let file = square.get_file().to_index() as f32;
        draw_texture_ex(
            texture,
            draw_info.x_start + file * draw_info.square_size,
            draw_info.y_start + (7. - rank) * draw_info.square_size,
            WHITE,
            draw_params.clone(),
        );
    }

    for square in board.pieces(Piece::Knight).into_iter() {
        let texture = match board.color_on(square).unwrap() {
            chess::Color::White => &textures.white_knight,
            chess::Color::Black => &textures.black_knight,
        };
        let rank = square.get_rank().to_index() as f32;
        let file = square.get_file().to_index() as f32;
        draw_texture_ex(
            texture,
            draw_info.x_start + file * draw_info.square_size,
            draw_info.y_start + (7. - rank) * draw_info.square_size,
            WHITE,
            draw_params.clone(),
        );
    }

    for square in board.pieces(Piece::Bishop).into_iter() {
        let texture = match board.color_on(square).unwrap() {
            chess::Color::White => &textures.white_bishop,
            chess::Color::Black => &textures.black_bishop,
        };
        let rank = square.get_rank().to_index() as f32;
        let file = square.get_file().to_index() as f32;
        draw_texture_ex(
            texture,
            draw_info.x_start + file * draw_info.square_size,
            draw_info.y_start + (7. - rank) * draw_info.square_size,
            WHITE,
            draw_params.clone(),
        );
    }

    for square in board.pieces(Piece::Rook).into_iter() {
        let texture = match board.color_on(square).unwrap() {
            chess::Color::White => &textures.white_rook,
            chess::Color::Black => &textures.black_rook,
        };
        let rank = square.get_rank().to_index() as f32;
        let file = square.get_file().to_index() as f32;
        draw_texture_ex(
            texture,
            draw_info.x_start + file * draw_info.square_size,
            draw_info.y_start + (7. - rank) * draw_info.square_size,
            WHITE,
            draw_params.clone(),
        );
    }

    for square in board.pieces(Piece::Queen).into_iter() {
        let texture = match board.color_on(square).unwrap() {
            chess::Color::White => &textures.white_queen,
            chess::Color::Black => &textures.black_queen,
        };
        let rank = square.get_rank().to_index() as f32;
        let file = square.get_file().to_index() as f32;
        draw_texture_ex(
            texture,
            draw_info.x_start + file * draw_info.square_size,
            draw_info.y_start + (7. - rank) * draw_info.square_size,
            WHITE,
            draw_params.clone(),
        );
    }

    for square in board.pieces(Piece::King).into_iter() {
        let texture = match board.color_on(square).unwrap() {
            chess::Color::White => &textures.white_king,
            chess::Color::Black => &textures.black_king,
        };
        let rank = square.get_rank().to_index() as f32;
        let file = square.get_file().to_index() as f32;
        draw_texture_ex(
            texture,
            draw_info.x_start + file * draw_info.square_size,
            draw_info.y_start + (7. - rank) * draw_info.square_size,
            WHITE,
            draw_params.clone(),
        );
    }
}
