use super::*;
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
}

pub fn add_map_to_world(mut commands: Commands) {
    commands.spawn(Map::basic(20, 20));
}
