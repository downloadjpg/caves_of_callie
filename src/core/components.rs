use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Renderable {
    pub glyph: char,
    pub fg: LinearRgba,
    pub bg: LinearRgba,
    //pub depth: i32
}

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct Position(pub IVec2);

impl From<[i32; 2]> for Position {
    fn from(p: [i32; 2]) -> Self {
        Position(IVec2::from(p))
    }
}

impl From<IVec2> for Position {
    fn from(v: IVec2) -> Self {
        Position(v.into())
    }
}
