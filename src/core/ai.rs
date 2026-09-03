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
    mut q_ai: Query<(Entity, &AiBehavior, &Position), With<Ready>>,
    player_pos: Option<Single<&Position, With<Player>>>,
    mut commands: Commands,
) {
    for (entity, _behavior, pos) in q_ai.iter_mut() {
        // let dist_to_player = player_pos.0 - pos.0;
        // let target = {
        //     if dist_to_player.length_squared() <= 2 {
        //         player_pos.0
        //     } else {
        //         pos.0 + pick_random_direction()
        //     }
        // };
        commands.entity(entity).insert(Intent(Action::Move {
            target: pos.0 + pick_random_direction(),
        }));
    }
}

fn pick_random_direction() -> IVec2 {
    let dirs = [IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y];
    let dir = dirs[rand::random::<u8>() as usize % dirs.len()];
    dir
}
