use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;
#[allow(dead_code)]
mod ai;
mod display;
mod log;
mod map;
mod monster;
mod movement;
mod player;
mod turn_system;

use map::*;
use movement::Position;

use crate::ai::AiPlugin;
use crate::display::DisplayPlugin;
use crate::log::AnnouncementLogPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use crate::turn_system::TurnSystemPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_plugins((
            TurnSystemPlugin,
            MapPlugin,
            DisplayPlugin,
            MovementPlugin,
            PlayerPlugin,
            AnnouncementLogPlugin,
            AiPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup).chain())
        .add_systems(Update, system_input)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn player
    commands.spawn(player::Player::default());
    commands.spawn((monster::Orc, Position([5, 5].into())));
    commands.spawn((monster::Orc, Position([2, 5].into())));
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
