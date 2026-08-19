use bevy::prelude::*;
use bevy_ascii_terminal::*;

#[derive(Event)]
pub struct Announcement(pub String);

#[derive(Component)]
pub struct AnnouncementLog(pub Vec<String>);

pub fn display_message(
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

pub fn announcement(msg: impl Into<String>) -> Announcement {
    Announcement(msg.into())
}
