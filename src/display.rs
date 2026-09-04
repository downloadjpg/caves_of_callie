use crate::core::components::{Position, Renderable};
use crate::core::log::AnnouncementLog;
use crate::core::map::*;

use bevy::prelude::*;
use bevy_ascii_terminal::*;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw);
        app.add_systems(Startup, startup);
    }
}

#[derive(Component)]
pub struct MapDisplay;

fn startup(mut commands: Commands) {
    commands.spawn(TerminalCamera::new());
    // Announcements Panel
    commands.spawn((
        AnnouncementLog::default(),
        Terminal::new([30, 40]).with_border(BoxStyle::SINGLE_LINE),
        Transform::from_xyz(30.0, 0.0, 0.0),
    ));
    // Map Display
    commands.spawn((
        crate::display::MapDisplay,
        Terminal::new([30, 40])
            .with_border(BoxStyle::SINGLE_LINE)
            // .TerminalMeshTileScaling(Vec2 { x: 1.0, y: 1.0 })
            .with_title("Caves of Callie"),
    ));
}

fn draw(
    mut term: Single<&mut Terminal, With<MapDisplay>>,
    q_map: Single<&Map>, // Map Resource?
    q_renderables: Query<(&Renderable, &Position)>, //, Without<Cursor>>,
                         // q_cursor: Query<(&Position, &Renderable), With<Cursor>>,
) {
    // Draw the level terrain
    q_map.draw(&mut term);
    // Draw renderable entities
    for (renderable, position) in q_renderables {
        if let Some(tile) = term.try_tile_mut(position.0) {
            tile.glyph = renderable.glyph;
            tile.bg_color = renderable.bg.into();
            tile.fg_color = renderable.fg.into();
        }
    }
    // TODO: Implement depth value on renderable.
    // // Draw cursors
    // for (position, renderable) in q_cursor {
    //     if let Some(tile) = term.try_tile_mut(position.0) {
    //         tile.glyph = renderable.glyph;
    //         tile.bg_color = renderable.bg.into();
    //         tile.fg_color = renderable.fg.into();
    //     }
    // }
}
