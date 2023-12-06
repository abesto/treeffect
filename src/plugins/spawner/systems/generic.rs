use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::Rng;

use crate::{
    components::{energy::Energy, position::Position},
    plugins::spawner::bundles::DogBundle,
};

pub enum SpawnType {
    Dog,
}

#[derive(Event)]
pub struct Spawn {
    pub xy: UVec2,
    pub kind: SpawnType,
}

pub fn spawn(
    mut commands: Commands,
    mut ev: EventReader<Spawn>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for ev in ev.read() {
        let bundle = match ev.kind {
            SpawnType::Dog => DogBundle {
                energy: Energy::new(rng.gen_range(1..=500)),
                ..Default::default()
            },
        };
        commands.spawn(bundle).insert(Position::from(ev.xy));
    }
}
