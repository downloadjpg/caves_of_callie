use bevy::prelude::*;

use crate::FONT_SIZE;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use crate::Map;
use crate::TILE_SIZE;

#[derive(Component)]
pub struct GridCell {
    x: i32,
    y: i32,
}

pub fn setup_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/mono.ttf");
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            commands.spawn((
                Text2d::new(" "),
                TextFont {
                    font: font.clone().into(),
                    font_size: FONT_SIZE.into(),
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    -(y as f32) * TILE_SIZE, // screen y grows downward
                    0.0,
                ),
                GridCell { x, y },
            ));
        }
    }
}

pub fn render_map_system(
    map: Res<Map>,
    mut query: Query<(&GridCell, &mut Text2d, &mut TextColor)>,
) {
    for (cell, mut text, mut color) in &mut query {
        if let Some(tile) = map.get(cell.x, cell.y) {
            *text = Text2d::new(tile.glyph());
            *color = TextColor(tile.color());
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    let t = Transform::from_xyz(
        (GRID_WIDTH as f32 * TILE_SIZE) / 2.0,
        -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0,
        0.0,
    );
    commands.spawn((Camera2d, t));
}
