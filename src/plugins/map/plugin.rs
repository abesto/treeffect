use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;

use crate::{
    consts::MAP_SIZE,
    plugins::spawner::events::{Spawn, SpawnPlayer},
};

pub struct MapPlugin {
    pub size: UVec2,
}

fn generate_map(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut ev_spawn_player: EventWriter<SpawnPlayer>,
    ev_spawn: EventWriter<Spawn>,
) {
    let mut builder = super::mapgen::random_builder(&mut *rng, MAP_SIZE.x, MAP_SIZE.y, 1);
    builder.build_map(&mut *rng);
    ev_spawn_player.send(SpawnPlayer {
        xy: builder.get_starting_position(),
    });
    builder.spawn_entities(ev_spawn, &mut *rng);
    let map = builder.get_map();
    commands.insert_resource(map);
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map);
    }
}
