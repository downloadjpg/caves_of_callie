use crate::map::Map;
use bevy::prelude::*;

const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 40;
const TILE_SIZE: f32 = 20.0;
const FONT_SIZE: f32 = 20.0;

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
                Text2d::new("p"),
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
    map_query: Query<&mut Map>,
    cell_query: Query<(&GridCell, &mut Text2d, &mut TextColor)>,
) {
    let map = map_query.iter().next().expect("No map found for game!");
    for (cell, mut text, mut color) in cell_query {
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
