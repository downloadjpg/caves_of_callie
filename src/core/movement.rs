use crate::core::{
    components::Position,
    log::announcement,
    map::Map,
    turn_system::{Action, Actor, Intent, Ready, TurnSet},
};
use bevy::prelude::*;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MoveMessage>();
        app.add_systems(Update, apply_move.in_set(TurnSet::Resolution));
    }
}

#[derive(Message)]
pub struct MoveMessage(pub Entity, pub IVec2);

fn apply_move(
    map: Single<&mut Map>,
    mover: Single<(Entity, &Intent, &mut Position), (With<Actor>, With<Ready>)>,
    other_actors: Query<&Position, (With<Actor>, Without<Ready>)>,
    mut commands: Commands,
) {
    let (_entity, intent, mut pos) = mover.into_inner();
    // Skip over non-move actions
    let Intent(Action::Move { target }) = intent else {
        return;
    };
    // Check if the spot is available on the map.
    if !map.is_walkable(*target) {
        return; // blocked - could emit an event instead...
    }
    // Check if we're bumping into a monster
    let occupied = other_actors.iter().any(|pos| pos.0 == *target);
    if occupied {
        commands.trigger(announcement("Bump!"));
        return;
    }
    // IDEA: Replace this occupancy check with an 'occupancy' resource that tracks where all occupying entities are.
    // Update position
    pos.0 = *target;
}
