use crate::{
    render::BACKGROUND_COLOR, GameMode, GameResult, GameRule, GameState, SideToMove, SwitchSides, TurnEndEvent
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, FontId, RichText},
    EguiContexts,
};

pub fn main_menu(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<GameState>>,
    mut game_mode: ResMut<NextState<GameMode>>,
    mut switch_sides: ResMut<SwitchSides>,
    mut up_ev: EventWriter<TurnEndEvent>,
    mut game_rule: ResMut<GameRule>,
) {
    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(301.0 - 100.0);
            ui.heading(RichText::new("Shake Chess").font(FontId::proportional(40.0)));

            let hotseat_button =
                ui.button(RichText::new("Play Hotseat").font(FontId::proportional(30.0)));
            let vs_ai_button =
                ui.button(RichText::new("Play vs. AI").font(FontId::proportional(30.0)));
            let ai_vs_ai_button =
                ui.button(RichText::new("Watch AI vs. AI").font(FontId::proportional(30.0)));

            if hotseat_button.clicked() {
                up_ev.send(TurnEndEvent);
                state.set(GameState::Playing);
                game_mode.set(GameMode::Hotseat);
                switch_sides.0 = true;
            }

            if vs_ai_button.clicked() {
                up_ev.send(TurnEndEvent);
                state.set(GameState::Playing);
                game_mode.set(GameMode::VsAi);
                switch_sides.0 = false;
            }

            if ai_vs_ai_button.clicked() {
                up_ev.send(TurnEndEvent);
                state.set(GameState::Playing);
                game_mode.set(GameMode::Sim);
                switch_sides.0 = false;
            }
        });
    });
}

pub fn end_screen(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<GameState>>,
    mut up_ev: EventWriter<TurnEndEvent>,
    mut board: ResMut<crate::game::Board>,
    mut last_50: ResMut<crate::Last50>,
    result: Res<GameResult>,
    mut drawn: Query<
        Entity,
        Or<(
            With<crate::Piece>,
            With<crate::Square>,
            With<crate::Selector>,
        )>,
    >,
) {
    let header = match *result {
        GameResult::Ongoing => unreachable!(),
        GameResult::Checkmate { winner } => format!("Winner: {:?}", winner),
        GameResult::Stalemate => String::from("Draw"),
    };
    let ctx = contexts.ctx_mut();
    egui::SidePanel::right("")
        .show_separator_line(false)
        .resizable(false)
        .exact_width(316.)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(301.0 - 100.0);
                ui.heading(RichText::new(header).font(FontId::proportional(40.0)));

                let go_again = ui.button(RichText::new("Restart").font(FontId::proportional(30.0)));
                let return_to_menu =
                    ui.button(RichText::new("Return to Menu").font(FontId::proportional(30.0)));

                if go_again.clicked() {
                    up_ev.send(TurnEndEvent);
                    state.set(GameState::Playing);
                    *board = crate::game::Board::default();
                    *last_50 = crate::Last50::default();
                    for e in drawn.iter() {
                        commands.entity(e).despawn_recursive();
                    }
                }

                if return_to_menu.clicked() {
                    *board = crate::game::Board::default();
                    *last_50 = crate::Last50::default();
                    state.set(GameState::MainMenu);
                    for e in drawn.iter() {
                        commands.entity(e).despawn_recursive();
                    }
                }
            });
        });
}

pub fn turn_readout(
    mut contexts: EguiContexts,
    side_to_move: Res<SideToMove>,
) {
    let ctx = contexts.ctx_mut();
    egui::SidePanel::right("")
        .show_separator_line(false)
        .resizable(false)
        .exact_width(316.)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let side = format!("{:?}'s Turn!", side_to_move.0);
                ui.heading(RichText::new(side).font(FontId::proportional(40.0)));
            });
        });
}
