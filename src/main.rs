use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;
mod components;
mod log;
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
        .add_systems(Update, (system_input, player::player_movement, draw))
        .run();
}

fn setup(mut commands: Commands) {
    // Create a terminal and a camera
    commands.spawn((
        map::MapDisplay,
        Terminal::new([30, 40])
            .with_border(BoxStyle::SINGLE_LINE)
            // .TerminalMeshTileScaling(Vec2 { x: 1.0, y: 1.0 })
            .with_title("Caves of Callie"),
    ));
    // Terminal for announcements/messages
    commands.spawn((
        log::AnnouncementLog(vec![String::from("first")]),
        Terminal::new([20, 40]).with_border(BoxStyle::SINGLE_LINE),
        Transform::from_xyz(30.0, 0.0, 0.0),
    ));
    commands.add_observer(log::display_message);
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

fn system_input(
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
