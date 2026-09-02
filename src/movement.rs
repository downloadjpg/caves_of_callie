use crate::Map;
use crate::turn_system::*;
use bevy::prelude::*;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_move.in_set(TurnSet::Resolution));
    }
}

#[derive(Message)]
pub struct MoveMessage(pub Entity, pub IVec2);

#[derive(Component, Clone, Copy, Default, Debug)]
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
    mut reader: MessageReader<MoveMessage>,
    map: Single<&mut Map>,
    mut q_actors: Query<(Entity, &mut Position), With<Actor>>,
) {
    for &MoveMessage(entity, new_pos) in reader.read() {
        if !map.is_walkable(new_pos) {
            continue; // blocked — could emit a MoveBlocked message here instead
        }

        // Check if we're bumping into a monster
        let occupied = q_actors
            .iter()
            .any(|(other, pos)| other != entity && pos.0 == new_pos);

        if occupied {
            continue;
        }

        // Update the entity's position
        let Ok((_, mut position)) = q_actors.get_mut(entity) else {
            continue; // entity despawned or has no Position, skip
        };
        *position = new_pos.into();
    }
}
