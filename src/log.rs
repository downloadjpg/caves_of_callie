use crate::Map;
use crate::turn_system::*;
use bevy::prelude::*;
use bevy_ascii_terminal::*;

pub struct AnnouncementLogPlugin;

impl Plugin for AnnouncementLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(AnnouncementLog::default());
        });
        app.add_systems(Update, announce_actions);
        app.add_observer(display_message);
    }
}

#[derive(Event)]
pub struct Announcement(pub String);

impl From<String> for Announcement {
    fn from(value: String) -> Self {
        Announcement(value.into())
    }
}

pub fn announcement(msg: impl Into<String>) -> Announcement {
    Announcement(msg.into())
}

#[derive(Component, Default)]
#[require(
    Terminal = Terminal::new([20, 40]).with_border(BoxStyle::SINGLE_LINE),
    Transform::from_xyz(30.0, 0.0, 0.0),
)]
struct AnnouncementLog(pub Vec<String>);

fn display_message(
    announcement: On<Announcement>,
    query: Single<(&mut Terminal, &mut AnnouncementLog)>,
) {
    let (mut term, mut log) = query.into_inner();
    let message_capacity = 10;
    log.0.push(announcement.0.clone());
    while log.0.len() > message_capacity {
        log.0.remove(0);
    }
    let messages = log.0.join("\n");
    term.clear();
    term.put_border(BoxStyle::SINGLE_LINE);
    term.put_string([0, 0], messages);
}

fn announce_actions(
    mut commands: Commands,
    mut msg: MessageReader<ActionPerformed>,
    map: Single<&Map>,
) {
    for performance in msg.read() {
        let entity = performance.entity;
        let action = performance.action;

        match action {
            Action::Move { new_pos } => {
                if !map.is_walkable(new_pos) {
                    commands.trigger(announcement("A wall!"))
                }
            }
            _ => {}
        }
    }
}
