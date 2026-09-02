use bevy::prelude::*;

use crate::core::monster::{Health, Stats};

/*
app.add_systems(Update, (
    resolve_attacks,      // AttackEvent -> compute hit/miss/damage -> DamageEvent
    apply_damage,         // DamageEvent -> mutate Health -> maybe DeathEvent
    handle_death,         // DeathEvent -> add Dead marker, drop loot, etc.
    cleanup_dead,         // despawn / remove from turn queue
).chain());
*/

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resolve_attacks);
    }
}

#[derive(Message)]
pub struct AttackMessage {
    pub attacker: Entity,
    pub target: Entity,
}
#[allow(dead_code)]
#[derive(Message)]
struct DamageMessage {
    target: Entity,
    amount: i32,
    source: Entity,
}

#[allow(dead_code)]
#[derive(Message)]
struct DeathMessage {
    entity: Entity,
}

fn resolve_attacks(
    mut attacks: MessageReader<AttackMessage>,
    mut _combatants: Query<(Entity, &mut Health, &Stats)>,
) {
    for _msg in attacks.read() {}
}
