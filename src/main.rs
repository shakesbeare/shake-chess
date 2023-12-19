use anyhow::Result;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        sample_count: 8,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let board = chess::Board::default();
    let textures = shake_chess::load_textures().await?;

    loop {
        // PRE UPDATE PHASE
        let draw_info = shake_chess::board::calculate_draw_info();
        shake_chess::board::draw_board(&draw_info);
        shake_chess::board::draw_pieces(board, &draw_info, &textures);

        // UPDATE PHASE

        // POST UPDATE PHASE

        next_frame().await;
    }
}
