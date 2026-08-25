use super::*;
use crate::ai::*;
use crate::display::Renderable;
use crate::turn_system::*;

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
    Position,
    Actor,
    AiBehavior::Wander,
    Speed(5),
)]
pub struct Orc;
