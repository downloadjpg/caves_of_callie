use crate::core::components::Position;
use crate::core::player::Player;
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
    player_pos: Single<&Position, With<Player>>,
    //map: Single<&Map>,
) {
    //let player_pos = player_pos.iter().next();
    for (_entity, _behavior, pos, mut next_action) in q_ai.iter_mut() {
        // If the player is in range, attack!
        let dist_to_player = player_pos.0 - pos.0;
        let target = {
            if dist_to_player.length_squared() <= 2 {
                player_pos.0
            } else {
                pos.0 + pick_random_direction()
            }
        };
        next_action.0 = Some(Action::Move { target: target })
    }
}

fn pick_random_direction() -> IVec2 {
    let dirs = [IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y];
    let dir = dirs[rand::random::<u8>() as usize % dirs.len()];
    dir
}
