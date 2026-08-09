use bevy::prelude::*;
mod rendering;

// --- Config -----------------------------------------------------------

const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 40;
const TILE_SIZE: f32 = 20.0;
const FONT_SIZE: f32 = 20.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn glyph(self) -> &'static str {
        match self {
            Tile::Floor => ".",
            Tile::Wall => "#",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Tile::Floor => Color::srgb(0.35, 0.35, 0.4),
            Tile::Wall => Color::srgb(0.8, 0.8, 0.85),
        }
    }
}

#[derive(Resource)]
pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Tile> {
        self.index(x, y).map(|i| self.tiles[i])
    }

    pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
        if let Some(i) = self.index(x, y) {
            self.tiles[i] = tile;
        }
    }
}

fn build_map() -> Map {
    let mut map = Map {
        width: GRID_WIDTH,
        height: GRID_HEIGHT,
        tiles: vec![Tile::Floor; (GRID_WIDTH * GRID_HEIGHT) as usize],
    };

    for x in 0..GRID_WIDTH {
        map.set(x, 0, Tile::Wall);
        map.set(x, GRID_HEIGHT - 1, Tile::Wall);
    }
    for y in 0..GRID_HEIGHT {
        map.set(0, y, Tile::Wall);
        map.set(GRID_WIDTH - 1, y, Tile::Wall);
    }

    map
}

// --- Main -----------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roguelike".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(build_map())
        .add_systems(
            Startup,
            (rendering::spawn_camera, rendering::setup_grid).chain(),
        )
        .add_systems(Update, rendering::render_map_system)
        .run();
}
