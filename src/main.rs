use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;
mod map;
mod player;

use map::*;
use player::*;

#[derive(Component)]
struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
}

#[derive(Component, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
impl From<&Position> for IVec2 {
    fn from(pos: &Position) -> IVec2 {
        IVec2 { x: pos.x, y: pos.y }
    }
}

impl Default for Position {
    fn default() -> Position {
        Position { x: 0, y: 0 }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup).chain())
        .add_systems(Update, (handle_input, player_movement, draw))
        .run();
}

fn setup(mut commands: Commands) {
    // Create a terminal and a camera
    commands.spawn(Terminal::new([110, 60]).with_border(BoxStyle::SINGLE_LINE));
    commands.spawn(TerminalCamera::new());
    // Create boxy level
    commands.spawn(map::Map::basic(25, 15));
    // Spawn player
    commands.spawn(player::Player::default());
}

fn draw(
    mut term: Single<&mut Terminal>,
    q_map: Single<&map::Map>,
    q_renderables: Query<(&Renderable, &Position)>,
) {
    // Draw the level terrain
    q_map.draw(&mut term);
    // Draw renderable entities
    for (renderable, position) in q_renderables {
        if let Some(tile) = term.try_tile_mut(position) {
            tile.glyph = renderable.glyph;
            tile.bg_color = renderable.bg.into();
            tile.fg_color = renderable.fg.into();
        }
    }
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut win: Single<&mut Window>,
    mut exit: MessageWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if input.just_pressed(KeyCode::KeyF) {
        if win.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current) {
            win.mode = WindowMode::Windowed;
        } else {
            win.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    }
}

fn player_movement(input: Res<ButtonInput<KeyCode>>, q_player: Single<(&Player, &mut Position)>) {
    let mut dir = IVec2::ZERO;
    if input.just_pressed(KeyCode::ArrowRight) {
        dir.x += 1;
    }
    if input.just_pressed(KeyCode::ArrowLeft) {
        dir.x -= 1;
    }
    if input.just_pressed(KeyCode::ArrowUp) {
        dir.y -= 1;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        dir.y += 1;
    }
    let (_, mut position) = q_player.into_inner();
    position.x += dir.x;
    position.y += dir.y;
}
