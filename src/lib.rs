#![allow(clippy::too_many_arguments)]

pub mod render;
pub mod game;

use bevy::prelude::{Component, Resource, States};

#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Square;

#[derive(Component)]
pub struct Selector;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    Checkmate {
        winner: chess::Color,
    },
    Stalemate,
}

#[derive(Resource)]
pub struct SideToMove(pub chess::Color);
