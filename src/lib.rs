#![allow(clippy::too_many_arguments)]

pub mod ai;
pub mod game;
pub mod render;
pub mod ui;

use bevy::prelude::{Component, Event, Resource, States};

#[derive(Event, Debug)]
pub struct TurnEndEvent;

#[derive(Resource, Debug)]
pub struct Last50 {
    counter: usize,
    data: [bool; 50],
}

impl Default for Last50 {
    fn default() -> Self {
        Self {
            counter: 0,
            data: [true; 50],
        }
    }
}

impl Last50 {
    pub fn push(&mut self, data: bool) {
        self.data[self.counter] = data;
        self.counter += 1;
        if self.counter >= 50 {
            self.counter = 0;
        }
    }

    /// Returns true if the game should end in a draw
    pub fn should_draw(&self) -> bool {
        !self.data.into_iter().reduce(|a, b| a | b).unwrap_or(false)
    }
}

#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Square;

#[derive(Component)]
pub struct Selector;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    End,
}

#[derive(Resource, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameResult {
    #[default]
    Ongoing,
    Checkmate {
        winner: chess::Color,
    },
    Stalemate,
}

#[derive(Resource)]
pub struct SideToMove(pub chess::Color);

#[derive(Resource)]
pub struct SwitchSides(pub bool);

#[derive(Resource)]
pub struct GameRule {
    ai_color: chess::Color,
}

impl Default for GameRule {
    fn default() -> Self {
        Self {
            ai_color: chess::Color::Black,
        }
    }
}

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameMode {
    #[default]
    Hotseat,
    VsAi,
    Sim,
}
