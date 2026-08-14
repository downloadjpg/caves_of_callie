use bevy::prelude::*;

#[derive(Component)]
#[require(
    crate::Position(IVec2 { x: 5, y: 5 }),
    crate::Renderable {
        glyph: 'M',
        fg: bevy::color::palettes::css::PALE_GREEN.into(),
        bg: Color::BLACK,
    }
)]
pub struct Creature;

#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component, Clone, Copy)]
pub struct Position(pub IVec2);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position(IVec2::new(x, y))
    }
}

impl Default for Position {
    fn default() -> Position {
        Position(IVec2::default())
    }
}

use std::ops::{Deref, DerefMut};
impl Deref for Position {
    type Target = IVec2;
    fn deref(&self) -> &IVec2 {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut IVec2 {
        &mut self.0
    }
}

use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Add<IVec2> for Position {
    type Output = Position;
    fn add(self, rhs: IVec2) -> Position {
        Position(self.0 + rhs)
    }
}

impl AddAssign<IVec2> for Position {
    fn add_assign(&mut self, rhs: IVec2) {
        self.0 += rhs;
    }
}

impl Sub<IVec2> for Position {
    type Output = Position;
    fn sub(self, rhs: IVec2) -> Position {
        Position(self.0 - rhs)
    }
}

impl SubAssign<IVec2> for Position {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.0 -= rhs;
    }
}

impl From<&Position> for IVec2 {
    fn from(v: &Position) -> Self {
        v.0
    }
}

impl From<Position> for IVec2 {
    fn from(v: Position) -> Self {
        Self::from(&v)
    }
}
