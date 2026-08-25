use super::get_input_direction;
use crate::color;
use crate::{Position, display::*, player::Player};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

const LOOK_KEY: KeyCode = KeyCode::Semicolon;
pub struct PlayerCommandsPlugin;
impl Plugin for PlayerCommandsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: make only run on player turn
        app.add_systems(Update, move_cursor); // run if in PlayerTurn::Look state
        app.add_systems(Update, toggle.run_if(input_just_pressed(LOOK_KEY))); // run if in playerturn state(?)
    }
}

#[derive(Component)]
#[require(Renderable {
    glyph: '_',
    fg: color::css::LIGHT_GRAY,
    bg: color::srgba_bytes(0, 0, 0, 0),
})]
pub struct Cursor {
    pos: IVec2,
}

fn move_cursor(mut cursor: Single<&mut Cursor>, input: Res<ButtonInput<KeyCode>>) {
    // poll for input, move accordingly
    let dir = get_input_direction(input);
    cursor.pos += dir;
}

fn toggle(player_position: Query<&Position, With<Player>>) {}
