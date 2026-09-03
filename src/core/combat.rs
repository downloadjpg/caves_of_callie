use bevy::{audio::PlaybackMode::Despawn, prelude::*};

use crate::core::{
    components::Dead,
    monster::{Health, Stats},
    turn_system::TurnSet,
};

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
        app.add_message::<AttackMessage>();
        app.add_message::<DamageMessage>();
        app.add_message::<DeathMessage>();
        app.add_systems(
            Update,
            (resolve_attacks, apply_damage, handle_death)
                .chain()
                .in_set(TurnSet::Resolution),
        );
        app.add_systems(Update, (cleanup_dead).in_set(TurnSet::CleanUp));
    }
}

#[derive(Message)]
pub struct AttackMessage {
    pub attacker: Entity,
    pub defender: Entity,
}
#[allow(dead_code)]
#[derive(Message, Debug, Clone, Copy)]
struct DamageMessage {
    target: Entity,
    amount: i32,
    source: Entity,
}

#[allow(dead_code)]
#[derive(Message)]
pub struct DeathMessage {
    pub entity: Entity,
}

fn resolve_attacks(
    mut attacks: MessageReader<AttackMessage>,
    mut damage: MessageWriter<DamageMessage>,
    combatants: Query<(Entity, &mut Health, &Stats)>,
) {
    for msg in attacks.read() {
        let Ok(attacker) = combatants.get(msg.attacker) else {
            continue;
        };
        let Ok(defender) = combatants.get(msg.defender) else {
            continue;
        };

        let dmg_amt = defender.2.attack;

        let message = DamageMessage {
            target: defender.0,
            amount: dmg_amt,
            source: attacker.0,
        };
        damage.write(message);
    }
}

fn apply_damage(
    mut damages: MessageReader<DamageMessage>,
    mut deaths: MessageWriter<DeathMessage>,
    mut healths: Query<&mut Health>,
) {
    for msg in damages.read() {
        if let Ok(mut hp) = healths.get_mut(msg.target) {
            hp.0 -= msg.amount;
            println!("{}", hp.0);
            if hp.0 <= 0 {
                deaths.write(DeathMessage { entity: msg.target });
            }
        }
    }
}

fn handle_death(mut commands: Commands, mut deaths: MessageReader<DeathMessage>) {
    for death in deaths.read() {
        commands.entity(death.entity).insert(Dead);
    }
}

fn cleanup_dead(mut commands: Commands, bring_out_yer_dead: Query<Entity, With<Dead>>) {
    for dead in bring_out_yer_dead {
        commands.entity(dead).despawn();
    }
}
