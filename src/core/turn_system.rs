use crate::core::{combat::AttackMessage, components::Position, movement::MoveMessage};
use bevy::prelude::*;
/// Plugin for managing turns and turn order.
/// To hook in, add systems to the decide and resolve system sets.
/// Adapted from https://github.com/sarkahn/bevy_roguelike/blob/main/src/turn_system.rs

pub struct TurnSystemPlugin;

impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                TurnSet::DetermineIntent,
                TurnSet::ResolveIntent,
                TurnSet::Resolution,
                TurnSet::Announcements,
                TurnSet::CleanUp,
            )
                .chain(),
        );
        app.add_systems(PreUpdate, begin_turn);
        app.add_systems(Update, (resolve_intent).in_set(TurnSet::ResolveIntent));
        app.add_systems(PostUpdate, end_turn);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnSet {
    DetermineIntent, // Any AI systems. Sets the ActionIntent component
    ResolveIntent,   // Don't add anything here.
    Resolution,      // Any passive system or effect.
    Announcements,   // stupid.
    CleanUp,         // Despawning entities, other destructive effects.
}

#[derive(Component, Default, Debug)]
#[require(Energy, Speed, ActionIntent)]
pub struct Actor;

#[derive(Component, Default)]
pub struct Energy(i32);

#[derive(Component)]
pub struct Speed(pub i32);
impl Default for Speed {
    fn default() -> Self {
        Speed(10)
    }
}
/// An actor is ready when their energy reaches/exceeds 100
#[derive(Component, Default, Debug)]
pub struct Ready;

/// None represents an actor that hasn't made up its mind yet.
/// If this is a player... that's fine.
#[derive(Component, Clone, Copy, Default)]
pub struct ActionIntent(pub Option<Action>);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Move { target: IVec2 },
    OpenDoor { target: IVec2 },
    CloseDoor { target: IVec2 },
    Wait,
}

impl Action {
    pub fn cost(&self) -> i32 {
        // Default
        100
    }
}

/// Progresses the energy of all waiting actors. Marks an actor as ready if their energy is at/above 100.
/// This *should* mean only one actor is ever ready per Update. Not sure if that's important or performant.
/// TODO: Consider turning this into a batch system, so all ready actors are executed together with order based on speed.
fn begin_turn(
    mut commands: Commands,
    mut q_waiting_actors: Query<(Entity, &mut Energy, &Speed), (With<Actor>, Without<Ready>)>,
    q_ready_actors: Query<&Actor, With<Ready>>,
) {
    // Don't do anything if there are ready actors. Or no waiting actors.
    if !q_ready_actors.is_empty() || q_waiting_actors.is_empty() {
        return;
    }

    // We want to wait as long as we need to for the next actor to be ready, so we loop.
    // TODO: Safety valve for actors not having enough speed...
    let mut done = false;
    let max_iter = 100;
    let mut iter = 0;
    while !done && iter < max_iter {
        for (entity, mut energy, speed) in q_waiting_actors.iter_mut() {
            if energy.0 >= 100 {
                commands.entity(entity).insert(Ready);
                done = true;
            }
            energy.0 += speed.0;
        }
        iter += 1;
        if iter > max_iter {
            error!("Safety valve exploded.");
        }
    }
}

/// Won't progress to the resolution phase (via message) until the readied actor has decided their next action.
/// Game pauses while you think!
fn resolve_intent(
    acting: Query<(Entity, &mut ActionIntent, &mut Energy), With<Ready>>,
    others: Query<(Entity, &Position), Without<Ready>>,
    mut moves: MessageWriter<MoveMessage>,
    mut attacks: MessageWriter<AttackMessage>,
) {
    for (entity, mut intent, mut energy) in acting {
        if let Some(action) = intent.0 {
            // Determine if we're bumping into an enemy. Emit an attack action if so. Otherwise, emit the move message.
            match action {
                Action::Move { target } => {
                    if let Some(obstructing_enemy) = others
                        .iter()
                        .filter(|(other, pos)| *other != entity && pos.0 == target)
                        .next()
                    {
                        attacks.write(AttackMessage {
                            attacker: entity,
                            defender: obstructing_enemy.0,
                        });
                    } else {
                        moves.write(MoveMessage(entity, target));
                    }
                }
                _ => {}
            }
            energy.0 -= action.cost();

            intent.0 = None;
        }
    }
}

/// Unmarks an actor as ready when their energy falls below 100.
fn end_turn(
    mut commands: Commands,
    q_ready_actors: Query<(Entity, &Energy), (With<Ready>, With<Actor>)>,
) {
    for (entity, energy) in q_ready_actors.iter() {
        if energy.0 < 100 {
            commands.entity(entity).remove::<Ready>();
        }
    }
}
