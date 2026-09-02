use crate::core::components::Position;
use crate::core::turn_system::*;
use bevy::prelude::*;
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ai_decide.in_set(TurnSet::DetermineIntent));
    }
}

#[derive(Component, Default)]
pub enum AiBehavior {
    #[default]
    Wander,
    //Pursue { target: IVec2 },
    //Flee,
    //Patrol,
}

// cute idea, have next action be private and create a setter function to handle invariance.
fn ai_decide(
    mut q_ai: Query<(Entity, &AiBehavior, &Position, &mut ActionIntent), With<Ready>>,
    //player_pos: Query<&Position, With<Player>>,
    //map: Single<&Map>,
) {
    //let player_pos = player_pos.iter().next();
    for (_entity, behavior, pos, mut next_action) in q_ai.iter_mut() {
        next_action.0 = match behavior {
            AiBehavior::Wander => Some(decide_wander(pos.0)),
            //_ => Some(Action::Wait), //AiBehavior::Pursue { target: target } => decide_pursue(pos.0, target), //AiBehavior::Flee { .. } => decide_flee(*pos, *player_pos),
        };
    }
}

fn decide_wander(pos: IVec2) -> Action {
    let dirs = [IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y];
    let dir = dirs[rand::random::<u8>() as usize % dirs.len()];
    Action::Move { target: dir + pos }
}
