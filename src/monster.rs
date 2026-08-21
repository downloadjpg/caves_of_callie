use super::*;
use crate::turn_system::*;
use bevy::{prelude::*, render::RenderSystems::Render};
pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decide_monster_action.in_set(TurnSet::Decide));
    }
}

#[derive(Component)]
pub struct Monster;

fn decide_monster_action(mut q_monster: Query<(&mut NextAction, &Position), With<Monster>>) {
    for (mut next_action, position) in q_monster.iter_mut() {
        next_action.0 = Some(Action::Move {
            target: (IVec2::new(1, 0) + position.0),
        });
    }
}

pub fn spawn_orc(mut commands: Commands, pos: IVec2) {
    commands.spawn((
        Monster,
        Renderable {
            glyph: 'O',
            fg: Color::linear_rgb(1.0, 0.0, 0.0),
            bg: Color::BLACK,
        },
        Position(pos),
        Actor,
        Speed(5),
    ));
}
