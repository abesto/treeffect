use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use itertools::Itertools;
use rand::seq::IteratorRandom;

use crate::{
    components::energy::{Active, Energy},
    consts::ACTION_COST,
    events::took_turn::TookTurn,
};

fn add_energy(mut query: Query<(Entity, &mut Energy)>) {
    let items = query.iter_mut().collect_vec();
    let Some(max_energy) = items
        .iter()
        .max_by_key(|(_, energy)| energy.amount)
        .map(|item| item.1.amount)
    else {
        return;
    };

    let energy_charged = ACTION_COST - max_energy;
    for (_, mut energy) in items {
        energy.amount += energy_charged;
    }
}

fn pick_next_actor(
    mut commands: Commands,
    query: Query<(Entity, &Energy)>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let candidates = query
        .iter()
        .filter(|(_, energy)| energy.amount >= ACTION_COST);
    let Some((next_actor, _)) = candidates.choose(&mut *rng) else {
        return;
    };

    println!("Next actor: {:?}", next_actor);

    commands.entity(next_actor).insert(Active);
}

fn no_active_actors(query: Query<&Active>) -> bool {
    query.iter().next().is_none()
}

fn took_turn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Energy), With<Active>>,
    mut ev_took_turn: EventReader<TookTurn>,
) {
    for (entity, mut energy) in query.iter_mut() {
        for event_entity in ev_took_turn.read() {
            assert_eq!(event_entity.actor, entity);
            energy.amount -= ACTION_COST;
            commands.entity(entity).remove::<Active>();
            println!("Actor {:?} took turn", entity);
        }
    }
}

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (add_energy, pick_next_actor)
                .chain()
                .run_if(no_active_actors),
        )
        .add_systems(Last, took_turn);
    }
}
