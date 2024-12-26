use anyhow::Result;
use bevy::{prelude::*, window::{WindowEvent, WindowResized}, winit::cursor::CursorIcon};
use shake_chess::GameState;

fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        .insert_resource(ClearColor(
            Srgba::hex(shake_chess::render::BACKGROUND_COLOR)?.into(),
        ))
        .insert_resource(shake_chess::render::DrawInfo::default())
        .insert_resource(shake_chess::game::Board::default())
        .insert_resource(shake_chess::game::PointedSquare::default())
        .insert_resource(shake_chess::game::SelectedPiece::None)
        .insert_resource(shake_chess::SideToMove(chess::Color::White))
        .insert_resource(shake_chess::SwitchSides(true))
        .init_state::<GameState>()
        .add_event::<shake_chess::game::SparseUpdateEvent>()
        .add_systems(PreStartup, shake_chess::render::update_draw_info)
        .add_systems(Startup, (setup, shake_chess::game::setup_game))
        .add_systems(
            Update,
            (
                shake_chess::render::update_draw_info,
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
                    shake_chess::game::check_end,
                    toggle_switch_sides,
                )
                    .run_if(in_state(GameState::Playing)),
            ),
        )
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut up_ev: EventWriter<shake_chess::game::SparseUpdateEvent>,
    mut window_ev: EventWriter<WindowResized>,
    window: Query<Entity, With<Window>>,
) {
    for w in window.iter() {
        commands
            .entity(w)
            .insert(CursorIcon::System(bevy::window::SystemCursorIcon::Default));
        window_ev.send(WindowResized {
            window: w,
            width: 1280.,
            height: 720.,
        });
    }
    commands.spawn(Camera2d);
    up_ev.send(shake_chess::game::SparseUpdateEvent);
}

fn toggle_switch_sides(mut up_ev: EventWriter<shake_chess::game::SparseUpdateEvent>, mut switch_sides: ResMut<shake_chess::SwitchSides>, input: Res<ButtonInput<KeyCode>>)  {
    if input.just_pressed(KeyCode::Space) {
        switch_sides.0 = !switch_sides.0;
        up_ev.send(shake_chess::game::SparseUpdateEvent);
    }
}
