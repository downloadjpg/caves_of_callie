use super::*;
use crate::ai::*;
use crate::display::Renderable;
use crate::turn_system::*;

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
    Name("Orc".into()),
    Position,
    AiBehavior::Wander,
    Speed(5),
)]
pub struct Orc;

#[derive(Component, Default)]
pub struct Name(String);

#[derive(Component, Default)]
pub struct Stats {
    attack: i32,
}
