use crate::core::{components::*, monster::*, turn_system::*};
use bevy::prelude::*;

//mod commands;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            player_input_system.run_if(in_state(TurnState::Paused)),
        );
        app.add_systems(Update, pause_for_player_system.after(begin_turn));
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

fn player_input_system(
    input: Res<ButtonInput<KeyCode>>,
    s_player: Single<(Entity, &Position), With<Player>>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut commands: Commands,
) {
    let (player, pos) = s_player.into_inner();
    if let Some(action) = decide_player_action(&input, pos.0) {
        commands.entity(player).insert(Intent(action));
        next_state.set(TurnState::Processing);
    }
}

fn pause_for_player_system(
    _query: Single<&Player, Added<Ready>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    next_state.set(TurnState::Paused);
}

/// Reads player input and returns an action. A result of None means the player hasn't pressed a relevant key.
fn decide_player_action(input: &Res<ButtonInput<KeyCode>>, player_pos: IVec2) -> Option<Action> {
    let dir = get_input_direction(input);
    let target = dir + player_pos;
    if dir == IVec2::ZERO {
        if input.just_pressed(KeyCode::Numpad5) {
            Some(Action::Wait)
        } else {
            None
        }
    } else {
        Some(Action::Move { target })
    }
}

fn get_input_direction(input: &Res<ButtonInput<KeyCode>>) -> IVec2 {
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
