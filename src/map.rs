use crate::Position;
use crate::Renderable;
use bevy::color::palettes::basic;
use bevy::prelude::*;
use bevy_ascii_terminal::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn glyph(self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
        }
    }

    pub fn fg(self) -> Color {
        match self {
            Tile::Floor => basic::GRAY.into(),
            Tile::Wall => basic::TEAL.into(),
        }
    }
    pub fn bg(self) -> Color {
        match self {
            Tile::Floor => Color::BLACK,
            Tile::Wall => Color::BLACK,
        }
    }
}

#[derive(Component)]
pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn index(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn pos(&self, index: usize) -> Option<(i32, i32)> {
        if index < 0 || index >= self.tiles.len() {
            None
        } else {
            let x: i32 = index as i32 % self.width;
            let y: i32 = index as i32 / self.width;
            assert!(self.index(x, y) == Some(index));
            Some((x, y))
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

    pub fn basic(width: i32, height: i32) -> Map {
        let mut map = Map {
            width: width,
            height: height,
            tiles: vec![Tile::Floor; (width * height) as usize],
        };

        for x in 0..width {
            map.set(x, 0, Tile::Wall);
            map.set(x, height - 1, Tile::Wall);
        }
        for y in 0..height {
            map.set(0, y, Tile::Wall);
            map.set(width - 1, y, Tile::Wall);
        }

        map
    }

    pub fn draw(&self, mut term: Single<&mut Terminal>) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let pos = self.pos(i);
            if let Some(term_tile) = term.try_tile_mut(pos.unwrap()) {
                term_tile.glyph = tile.glyph();
                term_tile.fg_color = tile.fg().into();
                term_tile.bg_color = tile.bg().into();
            }
        }
    }

    pub fn spawn_tiles(mut commands: Commands, map: Map) {
        for i in 0..(map.width * map.height) {
            let pos = map.pos(i as usize).unwrap();
            let tile = map.get(pos.0, pos.1).unwrap();
            commands.spawn((
                Renderable {
                    glyph: tile.glyph(),
                    fg: tile.fg().into(),
                    bg: tile.bg().into(),
                },
                Position { x: pos.0, y: pos.1 },
            ));
        }
    }
}
