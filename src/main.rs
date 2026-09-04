use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
};

use crate::{
    core::{
        ai::AiPlugin,
        combat::CombatPlugin,
        components::Position,
        log::AnnouncementLogPlugin,
        map::MapPlugin,
        monster,
        movement::MovementPlugin,
        player::{self, PlayerPlugin},
        turn_system::{IntentLog, TurnState, TurnSystemPlugin},
    },
    ui::DisplayPlugin,
};
mod core;
mod ui;

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
            CombatPlugin,
        ))
        .add_plugins((EguiPlugin::default()))
        .add_plugins(ResourceInspectorPlugin::<IntentLog>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup).chain())
        .add_systems(Update, system_input)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn player
    commands.spawn(player::Player::default());
    commands.spawn((monster::Orc, Position([2, 5].into())));
    commands.spawn((monster::Orc, Position([10, 5].into())));
    commands.spawn((monster::Orc, Position([7, 5].into())));
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
