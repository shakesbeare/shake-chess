use crate::{GameRule, Last50, SideToMove, TurnEndEvent};
use bevy::prelude::*;
use chess::{MoveGen, Piece};
use rand::seq::IteratorRandom;

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
