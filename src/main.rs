use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;

#[allow(dead_code)]
mod log;
mod map;
mod movement;
mod player;
mod turn_system;

use map::*;
use movement::Position;

use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use crate::turn_system::TurnSystemPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_plugins((TurnSystemPlugin, MapPlugin, MovementPlugin, PlayerPlugin))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup).chain())
        .add_systems(Update, (system_input, draw))
        .run();
}

fn setup(mut commands: Commands) {
    // Terminal for announcements/messages
    commands.spawn(log::AnnouncementLog(vec![]));
    commands.add_observer(log::display_message);
    // Create boxy level

    // Spawn player
    commands.spawn(player::Player::default());
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

fn draw(
    mut term: Single<(&MapDisplay, &mut Terminal)>,
    q_map: Single<&map::Map>,
    q_renderables: Query<(&Renderable, &Position)>,
) {
    let term = &mut term.1;
    // Draw the level terrain
    q_map.draw(term);
    // Draw renderable entities
    for (renderable, position) in q_renderables {
        if let Some(tile) = term.try_tile_mut(position.0) {
            tile.glyph = renderable.glyph;
            tile.bg_color = renderable.bg.into();
            tile.fg_color = renderable.fg.into();
        }
    }
}

fn system_input(
    input: Res<ButtonInput<KeyCode>>,
    mut win: Single<&mut Window>,
    mut exit: MessageWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if input.just_pressed(KeyCode::KeyF) {
        if win.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current) {
            win.mode = WindowMode::Windowed;
        } else {
            win.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    }
}
