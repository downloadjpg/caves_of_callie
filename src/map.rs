use bevy::color::palettes::{self, basic};
use bevy::prelude::*;
use bevy_ascii_terminal::*;

#[derive(Clone, Copy, PartialEq)] // NOT a component!!!
pub enum Tile {
    Floor,
    Wall,
    OpenDoor,
    ClosedDoor,
}

impl Tile {
    pub fn glyph(self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::OpenDoor => '▒',
            Tile::ClosedDoor => '-',
        }
    }

    pub fn fg(self) -> Color {
        match self {
            Tile::Floor => basic::GRAY.into(),
            Tile::Wall => basic::TEAL.into(),
            Tile::OpenDoor | Tile::ClosedDoor => palettes::tailwind::AMBER_200.into(),
        }
    }
    pub fn bg(self) -> Color {
        match self {
            Tile::Floor => Color::BLACK,
            Tile::Wall => Color::BLACK,
            Tile::OpenDoor | Tile::ClosedDoor => palettes::tailwind::GRAY_950.into(),
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
    pub fn new_empty(width: usize, height: usize) -> Map {
        Map {
            width: width as i32,
            height: height as i32,
            tiles: vec![Tile::Floor; width * height],
        }
    }
    pub fn new_solid(width: usize, height: usize) -> Map {
        Map {
            width: width as i32,
            height: height as i32,
            tiles: vec![Tile::Wall; width * height],
        }
    }
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn index(&self, x: i32, y: i32) -> Option<usize> {
        match self.in_bounds(x, y) {
            true => Some((y * self.width + x) as usize),
            false => None,
        }
    }

    pub fn pos(&self, index: usize) -> Option<(i32, i32)> {
        if index >= self.tiles.len() {
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

    pub fn draw(&self, term: &mut Terminal) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let pos = self.pos(i);
            if let Some(term_tile) = term.try_tile_mut(pos.unwrap()) {
                term_tile.glyph = tile.glyph();
                term_tile.fg_color = tile.fg().into();
                term_tile.bg_color = tile.bg().into();
            }
        }
    }

    // pub fn spawn_tiles(mut commands: Commands, map: Map) {
    //     for i in 0..(map.width * map.height) {
    //         let pos = map.pos(i as usize).unwrap();
    //         let tile = map.get(pos.0, pos.1).unwrap();
    //         commands.spawn((
    //             Renderable {
    //                 glyph: tile.glyph(),
    //                 fg: tile.fg().into(),
    //                 bg: tile.bg().into(),
    //             },
    //             Position(IVec2 { x: pos.0, y: pos.1 }),
    //         ));
    //     }
    // }
}

pub struct MapBuilder {
    map: Map,
}
impl MapBuilder {
    pub fn new(width: usize, height: usize) -> MapBuilder {
        MapBuilder {
            map: Map::new_empty(width, height),
        }
    }

    pub fn paint_rect(mut self, rect: IRect, tile: Tile) -> MapBuilder {
        for y in rect.min.y..=rect.max.y {
            for x in rect.min.x..=rect.max.x {
                self.map.set(x, y, tile);
            }
        }
        self
    }
    pub fn paint(mut self, pos: IVec2, tile: Tile) -> MapBuilder {
        self.paint_rect(IRect { min: pos, max: pos }, tile)
    }

    pub fn build(self) -> Map {
        self.map
    }
}
