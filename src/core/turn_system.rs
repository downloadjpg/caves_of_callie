use crate::core::{combat::AttackMessage, components::Position, movement::MoveMessage};
use bevy::prelude::*;
/// Plugin for managing turns and turn order.
/// To hook in, add systems to the decide and resolve system sets.
/// Adapted from https://github.com/sarkahn/bevy_roguelike/blob/main/src/turn_system.rs

pub struct TurnSystemPlugin;

impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TurnState>();
        app.configure_sets(
            Update,
            (
                TurnSet::BeginInitiative,
                TurnSet::DetermineIntent,
                TurnSet::ResolveIntent,
                TurnSet::Resolution,
                TurnSet::Announcements,
                TurnSet::CleanUp,
                TurnSet::EndInitiative,
            )
                .chain()
                .run_if(in_state(TurnState::Processing)),
        );
        app.add_systems(
            Update,
            (
                begin_turn.in_set(TurnSet::BeginInitiative),
                resolve_intent.in_set(TurnSet::ResolveIntent),
                end_turn.in_set(TurnSet::EndInitiative),
                debug,
            ),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnSet {
    BeginInitiative,
    DetermineIntent, // Any AI systems. Sets the ActionIntent component
    ResolveIntent,   // Don't add anything here.
    Resolution,      // Any passive system or effect.
    Announcements,   // stupid.
    CleanUp,         // Despawning entities, other destructive effects.
    EndInitiative,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum TurnState {
    #[default]
    Processing,
    Paused,
}

#[derive(Component, Default, Debug)]
#[require(Energy, Speed, Intent)]
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

/// Component representing the actor's next move.
#[derive(Component, Clone, Copy, Default, Debug)]
pub struct Intent(pub Action);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub enum Action {
    Move {
        target: IVec2,
    },
    OpenDoor {
        target: IVec2,
    },
    CloseDoor {
        target: IVec2,
    },
    #[default]
    Wait,
}

impl Action {
    pub fn cost(&self) -> i32 {
        // Default
        100
    }
}

fn debug(state: Res<State<TurnState>>) {
    println!("{:?}", state.get())
}

const ACTION_COST: i32 = 100;
/// Progresses the energy of all waiting actors. Marks an actor as ready if their energy is at/above 100.
/// This means ticks can progress with no actor being ready.
pub fn begin_turn(
    mut commands: Commands,
    mut q_waiting_actors: Query<(Entity, &mut Energy, &Speed), (With<Actor>, Without<Ready>)>,
    q_ready_actors: Query<&Actor, With<Ready>>,
) {
    for (entity, energy, _speed) in q_waiting_actors.iter() {
        println!("ID {}:, {}", entity, energy.0)
    }

    // Don't do anything if there are ready actors. Or no waiting actors.
    if !q_ready_actors.is_empty() || q_waiting_actors.is_empty() {
        return;
    }
    // Every actor gains energy according to their speed. We mark any with enough energy as ready.
    for (entity, mut energy, speed) in q_waiting_actors.iter_mut() {
        energy.0 += speed.0;
        if energy.0 >= ACTION_COST {
            commands.entity(entity).insert(Ready);
        }
    }
}

fn resolve_intent(
    acting: Query<(Entity, Option<&Intent>, &mut Energy), With<Ready>>,
    others: Query<(Entity, &Position), Without<Ready>>,
    mut moves: MessageWriter<MoveMessage>,
    mut attacks: MessageWriter<AttackMessage>,
    mut commands: Commands,
) {
    for (entity, intent, mut energy) in acting {
        // Determine if we're bumping into an enemy. Emit an attack action if so. Otherwise, emit the move message.
        // TODO: this redirection should perhaps be moved to the ai... you can bump into something on accident.
        println!("Entity {} is thinking about {:?}", entity, intent);
        if let Some(intent) = intent {
            match intent.0 {
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
            energy.0 -= intent.0.cost();
            commands.entity(entity).remove::<Intent>();
        } else {
            println!("no intent found!");
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
