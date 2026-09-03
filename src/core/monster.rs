use crate::core::{ai::*, components::*, turn_system::*};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Monster;

#[derive(Component, Default)]
#[require(
    Monster,
    Renderable {
        glyph: 'O',
        fg: Color::linear_rgb(1.0, 0.0, 0.0).into(),
        bg: Color::BLACK.into(),
    },
    Actor,
    Health(20),
    Stats,
    Name("Orc".into()),
    Position,
    AiBehavior::Wander,
    Speed(5),
)]
pub struct Orc;

#[derive(Component, Default)]
pub struct Stats {
    pub attack: i32,
}

#[derive(Component, Default)]
pub struct Health(pub i32);
