/// Plugin for managing turns and turn order.
/// One actor takes a turn every Update loop.
/// Adapted from https://github.com/sarkahn/bevy_roguelike/blob/main/src/turn_system.rs
use bevy::prelude::*;
pub struct TurnSystemPlugin;

impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, turn_begin_system);
        app.add_systems(PostUpdate, turn_end_system);
    }
}
#[derive(Component)]
pub struct Actor;

#[derive(Default, Component)]
pub struct Energy(i32);

#[derive(Default, Component)]
pub struct Speed(i32);

/// An actor is ready when their energy reaches/exceeds 100
#[derive(Default, Component)]
pub struct Ready;

/// Progresses the energy of all waiting actors. Marks an actor as ready if their energy is at/above 100.
/// This *should* mean only one actor is ever ready per Update. Not sure if that's important or performant.
fn turn_begin_system(
    mut commands: Commands,
    mut q_waiting_actors: Query<(Entity, &mut Energy, &Speed), (With<Actor>, Without<Ready>)>,
    q_ready_actors: Query<&Actor, With<Ready>>,
) {
    // Don't do anything if there are ready actors.
    if !q_ready_actors.is_empty() {
        return;
    }

    // We want to wait as long as we need to for the next actor to be ready, so we loop.
    let done = false;
    while !done {
        for (entity, mut energy, speed) in q_waiting_actors.iter_mut() {
            if energy.0 >= 100 {
                commands.entity(entity).insert(Ready);
                break;
            }
            energy.0 += speed.0;
        }
    }
}

/// Unmarks an actor as ready when their energy falls below 100.
fn turn_end_system(
    mut commands: Commands,
    q_ready_actors: Query<(Entity, &Energy), (With<Ready>, With<Actor>)>,
) {
    for (entity, energy) in q_ready_actors.iter() {
        if energy.0 < 100 {
            commands.entity(entity).remove::<Ready>();
        }
    }
}
