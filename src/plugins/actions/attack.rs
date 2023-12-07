use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::seq::IteratorRandom;

use crate::{
    components::{intents::attack::AttackIntent, name::Name, position::Position},
    events::took_turn::TookTurn,
};

fn attack(
    mut commands: Commands,
    mut q_actor: Query<(Entity, &Name, &Position, &AttackIntent)>,
    q_target: Query<(&Name, &Position)>, // TODO: replace by collision system caching positions
    mut ev_took_turn: EventWriter<TookTurn>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (entity, attacker_name, attacker_position, AttackIntent(attack_direction)) in
        q_actor.iter_mut()
    {
        commands.entity(entity).remove::<AttackIntent>();
        let attack_location = (attacker_position.xy.as_ivec2() + *attack_direction).as_uvec2();

        if let Some((target_name, _)) = q_target
            .iter()
            .filter(|(_, position)| position.xy == attack_location)
            .choose(&mut *rng)
        {
            println!("{} attacks {}", attacker_name.name, target_name.name);
        }

        ev_took_turn.send(entity.into());
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack);
    }
}
