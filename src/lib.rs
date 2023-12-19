pub mod board;
pub mod svg;

use macroquad::texture::Texture2D;
use macroquad::texture::load_texture;
use macroquad::texture::FilterMode;
use anyhow::Result;
use futures::join;

pub struct Textures {
    pub white_king: Texture2D,
    pub black_king: Texture2D,
    pub white_queen: Texture2D,
    pub black_queen: Texture2D,
    pub white_bishop: Texture2D,
    pub black_bishop: Texture2D,
    pub white_knight: Texture2D,
    pub black_knight: Texture2D,
    pub white_rook: Texture2D,
    pub black_rook: Texture2D,
    pub white_pawn: Texture2D,
    pub black_pawn: Texture2D,
}

pub async fn load_textures() -> Result<Textures> {
    let white_king = load_texture("assets/chess1.png");
    let black_king = load_texture("assets/chess7.png");
    let white_queen = load_texture("assets/chess2.png");
    let black_queen = load_texture("assets/chess8.png");
    let white_bishop = load_texture("assets/chess3.png");
    let black_bishop = load_texture("assets/chess9.png");
    let white_knight = load_texture("assets/chess4.png");
    let black_knight = load_texture("assets/chess10.png");
    let white_rook = load_texture("assets/chess5.png");
    let black_rook = load_texture("assets/chess11.png");
    let white_pawn = load_texture("assets/chess6.png");
    let black_pawn = load_texture("assets/chess12.png");

    let (
        white_king,
        black_king,
        white_queen,
        black_queen,
        white_bishop,
        black_bishop,
        white_knight,
        black_knight,
        white_rook,
        black_rook,
        white_pawn,
        black_pawn,
    ) = join!(
        white_king,
        black_king,
        white_queen,
        black_queen,
        white_bishop,
        black_bishop,
        white_knight,
        black_knight,
        white_rook,
        black_rook,
        white_pawn,
        black_pawn
    );

    let white_king = white_king?;
    let black_king = black_king?;
    let white_queen = white_queen?;
    let black_queen = black_queen?;
    let white_bishop = white_bishop?;
    let black_bishop = black_bishop?;
    let white_knight = white_knight?;
    let black_knight = black_knight?;
    let white_rook = white_rook?;
    let black_rook = black_rook?;
    let white_pawn = white_pawn?;
    let black_pawn = black_pawn?;

    white_king.set_filter(FilterMode::Linear);
    black_king.set_filter(FilterMode::Linear);
    white_queen.set_filter(FilterMode::Linear);
    black_queen.set_filter(FilterMode::Linear);
    white_bishop.set_filter(FilterMode::Linear);
    black_bishop.set_filter(FilterMode::Linear);
    white_knight.set_filter(FilterMode::Linear);
    black_knight.set_filter(FilterMode::Linear);
    white_rook.set_filter(FilterMode::Linear);
    black_rook.set_filter(FilterMode::Linear);
    white_pawn.set_filter(FilterMode::Linear);
    black_pawn.set_filter(FilterMode::Linear);

    Ok(Textures {
        white_king,
        black_king,
        white_queen,
        black_queen,
        white_bishop,
        black_bishop,
        white_knight,
        black_knight,
        white_rook,
        black_rook,
        white_pawn,
        black_pawn,
    })
}
