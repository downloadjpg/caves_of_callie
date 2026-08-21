use crate::Map;
use crate::turn_system::*;
use bevy::prelude::*;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_move.in_set(TurnSet::Resolve));
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct Position(pub IVec2);

impl From<[i32; 2]> for Position {
    fn from(p: [i32; 2]) -> Self {
        Position(IVec2::from(p))
    }
}

impl From<IVec2> for Position {
    fn from(v: IVec2) -> Self {
        Position(v.into())
    }
}

fn apply_move(
    mut reader: MessageReader<ActionPerformed>,
    map: Single<&mut Map>,
    mut q_actors: Query<(Entity, &mut Position), With<Actor>>,
) {
    for &ActionPerformed { entity, action } in reader.read() {
        let Action::Move { new_pos } = action else {
            continue; // not a move action, some other Resolve system handles it
        };
        if !map.is_walkable(new_pos) {
            continue; // blocked — could emit a MoveBlocked message here instead
        }
        let occupied = q_actors
            .iter()
            .any(|(other, pos)| other != entity && pos.0 == new_pos);

        if occupied {
            continue;
        }

        let Ok((_, mut position)) = q_actors.get_mut(entity) else {
            continue; // entity despawned or has no Position, skip
        };
        *position = new_pos.into();
    }
}
