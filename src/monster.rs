use super::*;
use crate::ai::*;
use crate::turn_system::*;
use bevy::{prelude::*, render::RenderSystems::Render};

#[derive(Component, Default)]

pub struct Monster;
#[derive(Component, Default)]
#[require(
    Monster,
    Renderable {
        glyph: 'O',
        fg: Color::linear_rgb(1.0, 0.0, 0.0),
        bg: Color::BLACK,
    },
    Position,
    Actor,
    AiBehavior::Wander,
    Speed(5),
)]
pub struct Orc;
