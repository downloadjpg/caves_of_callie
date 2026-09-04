use crate::core::{
    combat::*, map::Map, movement::MoveMessage, player::Player, turn_system::TurnSet,
};
use bevy::prelude::*;
use bevy_ascii_terminal::*;

pub struct AnnouncementLogPlugin;

impl Plugin for AnnouncementLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(AnnouncementLog::default());
        });
        app.add_systems(
            Update,
            (
                announce_player_wall_bumps,
                announce_attacks,
                announce_deaths,
            )
                .in_set(TurnSet::Announcements),
        );
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
pub struct AnnouncementLog(Vec<String>);

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

// fn announce_actions(
//     mut commands: Commands,
//     mut msg: MessageReader<ActionPerformed>,
//     map: Single<&Map>,
// ) {
//     for performance in msg.read() {
//         let entity = performance.entity;
//         let action = performance.action;

//         match action {
//             Action::Move { target: new_pos } => {
//                 if !map.is_walkable(new_pos) {
//                     commands.trigger(announcement("A wall!"))
//                 }
//             }
//             _ => {}
//         }
//     }
// }

fn announce_player_wall_bumps(
    mut commands: Commands,
    mut moves: MessageReader<MoveMessage>,
    player: Single<Entity, With<Player>>,
    map: Single<&Map>,
) {
    for MoveMessage(entity, new_pos) in moves.read() {
        if player.ne(entity) {
            continue;
        }
        if !map.is_walkable(*new_pos) {
            commands.trigger(announcement("You bump into a wall."));
        }
    }
}

fn announce_attacks(
    names: Query<&Name>,
    mut attacks: MessageReader<AttackMessage>,
    mut commands: Commands,
) {
    for msg in attacks.read() {
        let attacker = names.get(msg.attacker).unwrap();
        let defender = names.get(msg.defender).unwrap();
        commands.trigger(announcement(format!("{} attacks {}!", attacker, defender)));
    }
}

fn announce_deaths(
    names: Query<&Name>,
    mut deaths: MessageReader<DeathMessage>,
    mut commands: Commands,
) {
    for msg in deaths.read() {
        let default = Name::new("Something");
        let name = names.get(msg.entity).unwrap_or(&default);
        commands.trigger(announcement(format!("{} is killed!", name)));
    }
}
