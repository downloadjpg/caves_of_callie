use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;
mod map;
mod player;

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

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup).chain())
        .add_systems(Update, (handle_input, render))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Terminal::new([110, 60])
            .with_border(BoxStyle::SINGLE_LINE)
            .with_string([0, 0], "HELLO!!!")
            .with_string([10, 10], "BYE!!!"),
    );
    commands.spawn(TerminalCamera::new());
    // Create boxy level
    let level = map::Map::basic(25, 15);
    map::spawn_tiles(commands, level);
}

fn render(mut term: Single<&mut Terminal>, q_renderables: Query<(&Renderable, &Position)>) {
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
