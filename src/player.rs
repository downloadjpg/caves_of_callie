use crate::components::*;
use crate::log::*;
use crate::map::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(
    crate::Position(IVec2 { x: 5, y: 5 }),
    crate::Renderable {
        glyph: '@',
        fg: Color::WHITE,
        bg: Color::BLACK,
    }
)]
pub struct Player;

impl Default for Player {
    fn default() -> Player {
        Player
    }
}

pub fn player_movement(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&Player, &mut Position)>,
    mut map: Single<&mut Map>,
    q_creatures: Query<(&Creature, &Position), Without<Player>>,
) {
    let dir = get_player_input(input);
    if dir == IVec2::ZERO {
        return;
    }
    let (_, position) = player.into_inner();
    let position = position.into_inner();
    let new_pos = position.0 + dir;

    // Check for creatures
    for (_, position) in q_creatures {
        if position.0 == new_pos {
            commands.trigger(announcement("There's a creature!"));
            return;
        }
    }

    // Check availibility on the map
    match map.get(new_pos.x, new_pos.y) {
        Some(Tile::Floor) => {
            position.0 += dir;
        }
        Some(Tile::Wall) => {
            commands.trigger(announcement("There is a wall here."));
        }
        Some(Tile::ClosedDoor) => {
            map.set(new_pos.x, new_pos.y, Tile::OpenDoor);
        }
        Some(Tile::OpenDoor) => {
            position.0 += dir;
        }
        None => {}
    }
}

fn get_player_input(input: Res<ButtonInput<KeyCode>>) -> IVec2 {
    // Cardinal directions: arrows + numpad
    let right_keys = [KeyCode::ArrowRight, KeyCode::Numpad6];
    let left_keys = [KeyCode::ArrowLeft, KeyCode::Numpad4];
    let up_keys = [KeyCode::ArrowUp, KeyCode::Numpad8];
    let down_keys = [KeyCode::ArrowDown, KeyCode::Numpad2];

    // Diagonals: numpad only
    let up_left_keys = [KeyCode::Numpad7];
    let up_right_keys = [KeyCode::Numpad9];
    let down_left_keys = [KeyCode::Numpad1];
    let down_right_keys = [KeyCode::Numpad3];

    if input.any_just_pressed(up_left_keys) {
        IVec2::new(-1, -1)
    } else if input.any_just_pressed(up_right_keys) {
        IVec2::new(1, -1)
    } else if input.any_just_pressed(down_left_keys) {
        IVec2::new(-1, 1)
    } else if input.any_just_pressed(down_right_keys) {
        IVec2::new(1, 1)
    } else if input.any_just_pressed(up_keys) {
        IVec2::new(0, -1)
    } else if input.any_just_pressed(down_keys) {
        IVec2::new(0, 1)
    } else if input.any_just_pressed(left_keys) {
        IVec2::new(-1, 0)
    } else if input.any_just_pressed(right_keys) {
        IVec2::new(1, 0)
    } else {
        IVec2::ZERO
    }
}
