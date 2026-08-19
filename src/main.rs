use bevy::{ecs::reflect::ReflectMessageFns, prelude::*, window::WindowMode};
use bevy_ascii_terminal::{render::TerminalMeshTileScaling, *};
mod components;
mod map;
mod player;

use components::*;
use map::*;
use player::*;

#[derive(Component)]
struct MapDisplay;

#[derive(Component)]
struct AnnouncementLog(Vec<String>);

fn display_message(
    announcement: On<Announcement>,
    mut query: Single<(&mut Terminal, &mut AnnouncementLog)>,
) {
    let (mut term, mut log) = query.into_inner();
    let mesasge_capacity = term.inner_size();
    log.0.push(announcement.0.clone());
    while log.0.len() > 10 {
        log.0.remove(0);
    }
    let messages = log.0.join("\n");
    term.clear();
    term.put_border(BoxStyle::SINGLE_LINE);
    term.put_string([0, 0], messages);
}

#[derive(Event)]
struct Announcement(String);

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
        MapDisplay,
        Terminal::new([30, 40])
            .with_border(BoxStyle::SINGLE_LINE)
            // .TerminalMeshTileScaling(Vec2 { x: 1.0, y: 1.0 })
            .with_title("Caves of Callie"),
    ));
    // Terminal for announcements/messages
    commands.spawn((
        AnnouncementLog(vec![String::from("first")]),
        Terminal::new([20, 40]).with_border(BoxStyle::SINGLE_LINE),
        Transform::from_xyz(30.0, 0.0, 0.0),
    ));
    commands.add_observer(display_message);
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
        .paint(IVec2 { x: 10, y: 10 }, map::Tile::ClosedDoor)
        .build();
    commands.spawn(map);
    // Spawn player
    commands.spawn(player::Player::default());
    commands.spawn(components::Creature);
    commands.trigger(Announcement(
        "You see a giant huge frog or something!".into(),
    ));
}

fn draw(
    mut term: Single<(&MapDisplay, &mut Terminal)>,
    q_map: Single<&map::Map>,
    q_renderables: Query<(&Renderable, &Position)>,
) {
    let term = &mut term.1;
    // Draw the level terrain
    q_map.draw(term);
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
    for (creature, position) in q_creatures {
        if position.0 == new_pos {
            commands.trigger(Announcement("There's a creature!".into()));
            return;
        }
    }

    // Check availibility on the map
    match map.get(new_pos.x, new_pos.y) {
        Some(map::Tile::Floor) => {
            position.0 += dir;
        }
        Some(map::Tile::Wall) => {
            commands.trigger(Announcement("There is a wall here.".into()));
        }
        Some(map::Tile::ClosedDoor) => {
            map.set(new_pos.x, new_pos.y, map::Tile::OpenDoor);
        }
        Some(map::Tile::OpenDoor) => {
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
