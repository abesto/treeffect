use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::{
        energy::{Active, Energy},
        player::Player,
    },
    consts::ACTION_COST,
    events::took_turn::TookTurn,
};

fn add_energy(
    mut q_others: Query<&mut Energy, Without<Player>>,
    mut q_player: Query<&mut Energy, With<Player>>,
) {
    let Some(mut player_energy) = q_player.iter_mut().next() else {
        return;
    };
    if player_energy.amount >= ACTION_COST {
        return;
    }
    let to_add = ACTION_COST - player_energy.amount;

    player_energy.amount += to_add;
    for mut energy in q_others.iter_mut() {
        energy.amount += to_add;
    }
}

fn pick_next_actors(mut commands: Commands, query: Query<(Entity, &Energy)>) {
    for (entity, energy) in query.iter() {
        if energy.amount >= ACTION_COST {
            commands.entity(entity).insert(Active);
        }
    }
}

fn no_active_actors(query: Query<&Active>) -> bool {
    query.iter().next().is_none()
}

fn took_turn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Energy), With<Active>>,
    mut ev_took_turn: EventReader<TookTurn>,
) {
    let mut energy_by_entity: HashMap<_, _> = query.iter_mut().collect();
    for event in ev_took_turn.read() {
        let entity = event.actor;
        if let Some(energy) = energy_by_entity.get_mut(&entity) {
            energy.amount -= ACTION_COST;
        }
        commands.entity(entity).remove::<Active>();
    }
}

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (add_energy, pick_next_actors)
                .chain()
                .run_if(no_active_actors),
        )
        .add_systems(Last, took_turn);
    }
}
