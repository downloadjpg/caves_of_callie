use bevy::prelude::*;
/// Plugin for managing turns and turn order.
/// One actor takes a turn every Update loop.
/// Adapted from https://github.com/sarkahn/bevy_roguelike/blob/main/src/turn_system.rs

pub struct TurnSystemPlugin;

impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ActionPerformed>();
        app.configure_sets(
            Update,
            (TurnSet::Decide, TurnSet::Execute, TurnSet::Resolve).chain(),
        );
        app.add_systems(PreUpdate, begin_turn);
        app.add_systems(Update, (execute_action).in_set(TurnSet::Execute));
        app.add_systems(PostUpdate, end_turn);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnSet {
    Decide,  // Any AI systems. Sets the NextAction component
    Execute, // Don't add anything here. Systems involving an action use ActionPerformed message reader.
    Resolve, // Any passive system, like poison or light.
}

#[derive(Component, Default, Debug)]
#[require(Energy, Speed, NextAction)]
pub struct Actor;

#[derive(Component, Default)]
pub struct Energy(i32);

#[derive(Component)]
pub struct Speed(i32);
impl Default for Speed {
    fn default() -> Self {
        Speed(10)
    }
}
/// An actor is ready when their energy reaches/exceeds 100
#[derive(Component, Default, Debug)]
pub struct Ready;

#[derive(Component, Clone, Copy, Default)]
pub struct NextAction(pub Option<Action>);

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Move { target: IVec2 },
    OpenDoor { target: IVec2 },
    CloseDoor { target: IVec2 },
}

impl Action {
    pub fn cost(&self) -> i32 {
        // Default
        100
    }
}

#[derive(Message, Clone, Copy)]
pub struct ActionPerformed {
    pub entity: Entity,
    pub action: Action,
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
            iter += 1;
        }
    }
}

/// Won't progress to the resolution phase (via message) until the readied actor has decided their next action.
/// Game pauses while you think!
fn execute_action(
    mut q_actors: Query<(Entity, &mut NextAction, &mut Energy), With<Ready>>,
    mut writer: MessageWriter<ActionPerformed>,
) {
    for (entity, mut next_action, mut energy) in &mut q_actors {
        if let Some(action) = next_action.0 {
            energy.0 -= action.cost();
            writer.write(ActionPerformed { entity, action });
            next_action.0 = None;
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
