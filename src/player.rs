use bevy::prelude::*;

#[derive(Component)]
#[require(
    crate::Position { x: 5, y: 5},
    crate::Renderable {
        glyph: '@',
        fg: Color::WHITE,
        bg: Color::BLACK,
    }
)]
pub struct Player;

impl Default for Player {
    fn default() -> Player {
        Player
    }
}
