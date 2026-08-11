use bevy::prelude::*;
mod map;
mod rendering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roguelike".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                map::add_map_to_world,
                rendering::spawn_camera,
                rendering::setup_grid,
            )
                .chain(),
        )
        .add_systems(Update, rendering::render_map_system)
        .run();
}
