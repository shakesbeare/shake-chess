use std::{
    str::FromStr,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{GameRule, Last50, SideToMove, TurnEndEvent};
use bevy::prelude::*;
use chess::{MoveGen, Piece};
use futures::FutureExt;
use miniserde::{Deserialize, Serialize};
use rand::seq::IteratorRandom;

static NETWORK_MOVE: Mutex<NetworkGetMove> = Mutex::new(NetworkGetMove::Idle);

#[derive(Resource)]
pub enum NetworkGetMove {
    Idle,
    Pending,
    Ready(NetworkMove),
}

pub struct NetworkMove {
    from: chess::Square,
    to: chess::Square,
}

#[derive(Serialize, Deserialize)]
struct StockfishOnlineResponse {
    success: bool,
    evaluation: f32,
    mate: Option<i32>,
    bestmove: String,
    continuation: String,
}

impl StockfishOnlineResponse {
    fn into_network_move(self) -> NetworkMove {
        let move_san = self.bestmove.split(" ").nth(1).unwrap();
        let from = chess::Square::from_str(&move_san[0..2]).unwrap();
        let to = chess::Square::from_str(&move_san[2..4]).unwrap();
        NetworkMove { from, to }
    }
}

pub fn stockfish_move(
    mut up_ev: EventWriter<TurnEndEvent>,
    mut side_to_move: ResMut<SideToMove>,
    game_rule: Res<GameRule>,
    mut board: ResMut<crate::game::Board>,
    mut last_50: ResMut<Last50>,
) {
    if side_to_move.0 != game_rule.ai_color {
        return;
    }

    let mut network_get_move = NETWORK_MOVE.lock().expect("Mutex should never be poisoned");

    match *network_get_move {
        NetworkGetMove::Idle => {
            request_network_move(board.to_string(), network_get_move);
        }
        NetworkGetMove::Pending => {}
        NetworkGetMove::Ready(ref network_move) => {
            // assume that the network move is well-formed
            let m = chess::ChessMove::new(network_move.from, network_move.to, None);
            let p = board.piece_on(m.get_source()).unwrap();
            let t = board.piece_on(m.get_dest());
            **board = board.make_move_new(m);
            side_to_move.0 = !side_to_move.0;

            up_ev.send(TurnEndEvent);
            last_50.push(p == Piece::Pawn || t.is_some());
            *network_get_move = NetworkGetMove::Idle;
        }
    }
}

pub fn request_network_move(fen: String, mut mutex_guard: MutexGuard<'_, NetworkGetMove>) {
    *mutex_guard = NetworkGetMove::Pending;
    drop(mutex_guard);
    crate::run_async(async move {
        let mut fut = tokio::task::spawn_local(async move {
            let client = reqwest::Client::new();
            let url = format!("https://stockfish.online/api/s/v2.php?fen={}&depth=12", fen);
            let res = client.get(url).send().await.unwrap();
            let text = res.text().await.unwrap();
            let stockfish_response: Result<StockfishOnlineResponse, _> =
                miniserde::json::from_str(&text);
            stockfish_response.unwrap()
        })
        .fuse();

        futures::select! {
            res = fut => {
                let net_move = NetworkGetMove::Ready(res.unwrap().into_network_move());
                *NETWORK_MOVE.lock().unwrap() = net_move;
            }
        }
    });
}

pub fn single_ai_move(
    mut up_ev: EventWriter<TurnEndEvent>,
    mut side_to_move: ResMut<SideToMove>,
    game_rule: Res<GameRule>,
    mut board: ResMut<crate::game::Board>,
    mut last_50: ResMut<Last50>,
) {
    if side_to_move.0 != game_rule.ai_color {
        return;
    }
    let move_gen = MoveGen::new_legal(board.as_ref());
    let mut rng = rand::thread_rng();
    let m = move_gen.into_iter().choose(&mut rng).unwrap();
    let p = board.piece_on(m.get_source()).unwrap();
    let t = board.piece_on(m.get_dest());
    **board = board.make_move_new(m);
    side_to_move.0 = !side_to_move.0;

    up_ev.send(TurnEndEvent);
    last_50.push(p == Piece::Pawn || t.is_some());
}

pub fn sim_ai_move(
    mut up_ev: EventWriter<TurnEndEvent>,
    mut side_to_move: ResMut<SideToMove>,
    mut board: ResMut<crate::game::Board>,
    mut last_50: ResMut<Last50>,
) {
    let move_gen = MoveGen::new_legal(board.as_ref());
    let mut rng = rand::thread_rng();
    let m = move_gen.into_iter().choose(&mut rng).unwrap();
    let p = board.piece_on(m.get_source()).unwrap();
    let t = board.piece_on(m.get_dest());

    **board = board.make_move_new(m);
    side_to_move.0 = !side_to_move.0;
    up_ev.send(TurnEndEvent);
    last_50.push(p == Piece::Pawn || t.is_some());
}
