use crate::core::{components::*, monster::*, turn_system::*};
use bevy::prelude::*;

//mod commands;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            decide_player_action.in_set(TurnSet::DetermineIntent),
        );
    }
}

#[derive(Default, Component)]
#[require(
    Actor,
    Name("Player".into()),
    Health,
    Stats { attack: 5},
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
    q_player: Single<(&mut ActionIntent, &Position), With<Player>>,
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
    input
        .get_just_pressed()
        .map(|key| match key {
            // Cardinal directions: arrows + numpad
            KeyCode::ArrowUp | KeyCode::Numpad8 => IVec2::new(0, -1), // Up
            KeyCode::ArrowDown | KeyCode::Numpad2 => IVec2::new(0, 1), // Down
            KeyCode::ArrowLeft | KeyCode::Numpad4 => IVec2::new(-1, 0), // Left
            KeyCode::ArrowRight | KeyCode::Numpad6 => IVec2::new(1, 0), // Right
            // Diagonals: numpad only
            KeyCode::Numpad7 => IVec2::new(-1, -1), // Up-Left
            KeyCode::Numpad9 => IVec2::new(1, -1),  // Up-Right
            KeyCode::Numpad1 => IVec2::new(-1, 1),  // Down-Left
            KeyCode::Numpad3 => IVec2::new(1, 1),   // Down-Right

            _ => IVec2::ZERO,
        })
        .sum()
}
