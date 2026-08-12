use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::{render::TerminalMeshTileScaling, *};
mod components;
mod map;
mod player;

use components::*;
use map::*;
use player::*;

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
    commands.spawn((
        Terminal::new([30, 40])
            .with_border(BoxStyle::SINGLE_LINE)
            .with_title("Caves of Callie"),
        TerminalMeshTileScaling(Vec2 { x: 1.0, y: 1.0 }),
    ));
    commands.spawn(TerminalCamera::new());
    // Create boxy level
    let map = map::MapBuilder::new(25, 15)
        .paint_rect(
            IRect::from_corners(IVec2::new(0, 0), IVec2::new(25, 15)),
            map::Tile::Wall,
        )
        .paint_rect(
            IRect::from_corners(IVec2::new(2, 2), IVec2::new(20, 10)),
            map::Tile::Floor,
        )
        .build();
    commands.spawn(map);
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

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&Player, &mut Position)>,
    map: Single<&Map>,
    //q_creatures: Query<(&Creature, &Position), Without<Player>>,
) {
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

    let dir = if input.any_just_pressed(up_left_keys) {
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
    };
    if dir == IVec2::ZERO {
        return;
    }
    let (_, position) = player.into_inner();
    let position = position.into_inner();
    let new_pos = position.0 + dir;
    match map.get(new_pos.x, new_pos.y) {
        Some(map::Tile::Floor) => {
            position.0 += dir;
        }
        Some(map::Tile::Wall) => {}
        None => {}
    }
}
