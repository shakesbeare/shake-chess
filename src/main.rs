use anyhow::Result;
use bevy::{prelude::*, window::PrimaryWindow, winit::cursor::CursorIcon};

fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        .insert_resource(ClearColor(
            Srgba::hex(shake_chess::render::BACKGROUND_COLOR)?.into(),
        ))
        .insert_resource(shake_chess::render::DrawInfo::default())
        .insert_resource(shake_chess::game::Board::default())
        .add_event::<shake_chess::game::MoveEvent>()
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
                shake_chess::render::mouse_hover,
            ),
        )
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut move_ev: EventWriter<shake_chess::game::MoveEvent>,
    window: Query<Entity, With<Window>>,
) {
    for w in window.iter() {
        commands
            .entity(w)
            .insert(CursorIcon::System(bevy::window::SystemCursorIcon::Default));
    }
    commands.spawn(Camera2d);
    move_ev.send(shake_chess::game::MoveEvent(None));
}

