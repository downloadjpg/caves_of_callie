use bevy::{ecs::relationship::OrderedRelationshipSourceCollection, prelude::*};
/// Plugin for managing turns and turn order.
/// To hook in, add systems to the decide and resolve system sets.
/// Adapted from https://github.com/sarkahn/bevy_roguelike/blob/main/src/turn_system.rs

pub struct TurnSystemPlugin;

impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TurnState>()
            .init_resource::<IntentLog>()
            .register_type::<IntentLog>()
            .register_type::<Intent>()
            .configure_sets(
                Update,
                (
                    TurnSet::BeginInitiative,
                    TurnSet::DetermineIntent,
                    TurnSet::Resolution,
                    TurnSet::Announcements,
                    TurnSet::CleanUp,
                    TurnSet::EndInitiative,
                )
                    .chain()
                    .run_if(in_state(TurnState::Processing)),
            )
            .add_systems(
                Update,
                (
                    begin_turn.in_set(TurnSet::BeginInitiative),
                    end_turn.in_set(TurnSet::EndInitiative),
                ),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnSet {
    BeginInitiative,
    DetermineIntent, // Any AI systems. Sets the ActionIntent component
    Resolution,      // Any passive system or effect.
    Announcements,   // stupid.
    CleanUp,         // Despawning entities, other destructive effects.
    EndInitiative,
}

#[derive(Clone, Copy, Reflect, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum TurnState {
    #[default]
    Processing,
    Paused,
}

#[derive(Component, Default, Debug)]
#[require(Energy, Speed, Intent)]
pub struct Actor;

#[derive(Component, Default, Ord, PartialOrd, PartialEq, Eq)]
pub struct Energy(i32);

#[derive(Component, PartialEq, PartialOrd, Ord, Eq)]
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
#[derive(Component, Reflect, Clone, Copy, Default, Debug)]
pub struct Intent(pub Action);

#[derive(Resource, Reflect, Debug, Default)]
pub struct IntentLog(Vec<(Entity, Intent)>);

#[allow(dead_code)]
#[derive(Clone, Reflect, Copy, Debug, Default)]
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

const ACTION_COST: i32 = 100;
/// Progresses the energy of all waiting actors. Marks an actor as ready if their energy is at/above 100.
/// This means ticks can progress with no actor being ready.
pub fn begin_turn(
    mut commands: Commands,
    mut q_waiting_actors: Query<(Entity, &mut Energy, &Speed), (With<Actor>, Without<Ready>)>,
    q_ready_actors: Query<&Actor, With<Ready>>,
) {
    // Don't do anything if there are ready actors. Or no waiting actors.
    if !q_ready_actors.is_empty() || q_waiting_actors.is_empty() {
        return;
    }
    // Every actor gains energy according to their speed.
    for (_, mut energy, speed) in q_waiting_actors.iter_mut() {
        energy.0 += speed.0;
    }
    // Select one actor to take a turn, adding the Ready component to it.
    if let Some((next_actor, _, _)) = q_waiting_actors
        .iter()
        .sort::<(&Energy, &Speed)>()
        .filter(|(_, energy, _)| energy.0 >= ACTION_COST)
        .next()
    {
        commands.entity(next_actor).insert(Ready);
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
