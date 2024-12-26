use anyhow::Result;
use bevy::{
    prelude::*,
    window::{WindowEvent, WindowResized},
    winit::cursor::CursorIcon,
};
use bevy_egui::{egui::self, EguiContexts};
use shake_chess::GameState;

fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .insert_resource(ClearColor(
            Srgba::hex(shake_chess::render::BACKGROUND_COLOR)?.into(),
        ))
        .insert_resource(shake_chess::render::DrawInfo::default())
        .insert_resource(shake_chess::game::Board::default())
        .insert_resource(shake_chess::game::PointedSquare::default())
        .insert_resource(shake_chess::game::SelectedPiece::None)
        .insert_resource(shake_chess::SideToMove(chess::Color::White))
        .insert_resource(shake_chess::SwitchSides(true))
        .insert_resource(shake_chess::GameRule::default())
        .insert_resource(shake_chess::Last50::default())
        .insert_resource(shake_chess::GameResult::default())
        .init_state::<GameState>()
        .init_state::<shake_chess::GameMode>()
        .add_event::<shake_chess::TurnEndEvent>()
        .add_systems(PreStartup, shake_chess::render::update_draw_info)
        .add_systems(Startup, (setup, setup_ui, shake_chess::game::setup_game))
        .add_systems(PreUpdate, (shake_chess::render::update_draw_info))
        .add_systems(
            Update,
            (
                (
                    shake_chess::render::draw_chessboard,
                    shake_chess::render::draw_pieces,
                )
                    .chain(),
                (
                    shake_chess::game::mouse_point,
                    shake_chess::game::act,
                    shake_chess::render::cursor_swap,
                    shake_chess::render::render_selector,
                    shake_chess::ai::single_ai_move.run_if(
                        in_state(shake_chess::GameState::Playing)
                            .and(in_state(shake_chess::GameMode::VsAi)),
                    ),
                    shake_chess::ai::sim_ai_move.run_if(
                        in_state(shake_chess::GameState::Playing)
                            .and(in_state(shake_chess::GameMode::Sim)),
                    ),
                    toggle_switch_sides,
                )
                    .run_if(in_state(GameState::Playing)),
            )
                .run_if(not(in_state(GameState::MainMenu))),
        )
        .add_systems(
            PostUpdate,
            shake_chess::game::check_end.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (shake_chess::ui::main_menu,).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            shake_chess::ui::end_screen.run_if(in_state(GameState::End)),
        )
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut up_ev: EventWriter<shake_chess::TurnEndEvent>,
    mut window_ev: EventWriter<WindowResized>,
    mut window: Query<(Entity, &mut Window)>,
) {
    for (e, mut window) in window.iter_mut() {
        commands
            .entity(e)
            .insert(CursorIcon::System(bevy::window::SystemCursorIcon::Default));
        window.resize_constraints = WindowResizeConstraints {
            min_width: 1280.,
            min_height: 720.,
            max_width: f32::MAX,
            max_height: f32::MAX,
        };
        window_ev.send(WindowResized {
            window: e,
            width: 1280.,
            height: 720.,
        });
    }
    commands.spawn(Camera2d);
    up_ev.send(shake_chess::TurnEndEvent);
}

pub fn setup_ui(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        panel_fill: egui::Color32::TRANSPARENT,
        ..default()
    });
}

fn toggle_switch_sides(
    mut up_ev: EventWriter<shake_chess::TurnEndEvent>,
    mut switch_sides: ResMut<shake_chess::SwitchSides>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        switch_sides.0 = !switch_sides.0;
        up_ev.send(shake_chess::TurnEndEvent);
    }
}
