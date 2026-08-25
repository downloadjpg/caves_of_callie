use crate::Position;
use crate::display::Renderable;
use crate::turn_system::*;
use bevy::prelude::*;

mod commands;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decide_player_action.in_set(TurnSet::Decide));
    }
}

#[derive(Default, Component)]
#[require(
    Actor,
    Position(IVec2 { x: 5, y: 5 }),
    Renderable {
        glyph: '@',
        fg: Color::WHITE.into(),
        bg: Color::BLACK.into(),
    }
)]

pub struct Player;

fn decide_player_action(
    input: Res<ButtonInput<KeyCode>>,
    q_player: Single<(&mut NextAction, &Position), With<Player>>,
) {
    let (mut next_action, position) = q_player.into_inner();
    let dir = get_input_direction(input);
    let action = if dir == IVec2::ZERO {
        None
    } else {
        Some(Action::Move {
            target: dir + position.0,
        })
    };
    next_action.0 = action;
}

fn get_input_direction(input: Res<ButtonInput<KeyCode>>) -> IVec2 {
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
