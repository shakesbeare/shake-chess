#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub mod ai;
pub mod game;
pub mod render;
pub mod ui;

use bevy::prelude::{Component, Event, Resource, States};
use futures::Future;

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

#[cfg(not(target_arch = "wasm32"))]
pub fn run_async<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Cannot start tokio runtime");

        rt.block_on(async move {
            let local = tokio::task::LocalSet::new();
            local
                .run_until(async move {
                    tokio::task::spawn_local(future).await.unwrap();
                })
                .await;
        });
    });
}

#[cfg(target_arch = "wasm32")]
pub fn run_async<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                tokio::task::spawn_local(future).await.unwrap();
            })
            .await;
    });
}
