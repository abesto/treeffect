use bevy::prelude::*;

use crate::{components::position::Position, plugins::spawner::bundles::DogBundle};

pub enum SpawnType {
    Dog,
}

#[derive(Event)]
pub struct Spawn {
    pub xy: UVec2,
    pub kind: SpawnType,
}

pub fn spawn(mut commands: Commands, mut ev: EventReader<Spawn>) {
    for ev in ev.read() {
        let bundle = match ev.kind {
            SpawnType::Dog => DogBundle::default(),
        };
        commands.spawn(bundle).insert(Position::from(ev.xy));
    }
}
