use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::seq::IteratorRandom;

use crate::{
    behavior,
    components::{name::Name, position::Position},
    events::{intents::attack::AttackIntent, took_turn::TookTurn},
};

fn attack(
    q_actor: Query<(&Name, &Position)>,
    attack: Listener<AttackIntent>,
    q_target: Query<(&Name, &Position)>, // TODO: replace by collision system caching positions
    mut ev_took_turn: EventWriter<TookTurn>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let attacker = attack.listener();
    let attack_direction = attack.vector;
    let Ok((attacker_name, attacker_position)) = q_actor.get(attacker) else {
        return;
    };
    let attack_location = (attacker_position.xy.as_ivec2() + attack_direction).as_uvec2();

    if let Some((target_name, _)) = q_target
        .iter()
        .filter(|(_, position)| position.xy == attack_location)
        .choose(&mut *rng)
    {
        println!("{} attacks {}", attacker_name, target_name);
    }

    ev_took_turn.send(attacker.into());
}

behavior!(attack);
