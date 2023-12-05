use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;

use crate::consts::MAP_SIZE;

use super::map::{Map, TileType};

pub struct MapPlugin {
    pub size: UVec2,
}

fn generate_map(mut commands: Commands, mut rng: ResMut<GlobalEntropy<WyRand>>) {
    let mut builder = super::mapgen::random_builder(&mut *rng, MAP_SIZE.x, MAP_SIZE.y, 1);
    builder.build_map(&mut *rng);
    let map = builder.get_map();
    commands.insert_resource(map);
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map);
    }
}
