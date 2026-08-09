use bevy::{prelude::*, window::WindowResolution};

/// World, has many objects in it. Objects live on a grid. Grid is rendered via tiles. So the renderer talks to the grid, gets all the renderables at that point.
///

// --- Config
// -----------------------------------------------------------

const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 40;
const CELL_SIZE: f32 = 20.0; // pixels between glyph centers
const FONT_SIZE: f32 = 20.0;

/// Marks an entity as one cell of the map grid,
/// storing its grid coords.
#[derive(Component)]
struct Position(Vec2);

/// Anything that can be rendered!
#[derive(Component)]
struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
}

/// One of the cells used to display the game world. Is bundled with a sprite and updates its visuals every render pass.
#[derive(Component)]
struct GridCell(Vec2);

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Wall,
}

impl Tile {
    fn glyph(self) -> &'static str {
        match self {
            Tile::Floor => ".",
            Tile::Wall => "#",
        }
    }

    fn color(self) -> Color {
        match self {
            Tile::Floor => Color::srgb(0.35, 0.35, 0.4),
            Tile::Wall => Color::srgb(0.8, 0.8, 0.85),
        }
    }
}

// --- Systems
// -----------------------------------------------------------

fn render(mut query: Query<&mut Text2d, With<GridCell>>) {
    // Get tile data from the map... hmmm.
    // Query for renderables with a position equal to something?
    // Query through all renderables?
}

// --- Main
// -----------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roguelike".into(),
                // resolution: WindowResolution::new(physical_width, physical_height),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_grid).chain())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_grid(mut commands: Commands) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let is_edge = x == 0 || y == 0 || x == GRID_WIDTH - 1 || y == GRID_HEIGHT - 1;
            let tile = if is_edge { Tile::Wall } else { Tile::Floor };

            commands.spawn((
                Text2d::new(tile.glyph()),
                TextFont {
                    font_size: FONT_SIZE.into(),
                    ..default()
                },
                TextColor(tile.color()),
                Transform::from_xyz(world_x, world_y, 0.0),
                GridCell(Vec2 {
                    x: x as f32,
                    y: y as f32,
                }),
            ));
        }
    }
}
