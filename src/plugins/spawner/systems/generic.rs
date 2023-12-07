use paste::paste;
use std::cmp::max;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::{seq::IteratorRandom, Rng};

use crate::{
    components::{energy::Energy, position::Position},
    plugins::spawner::bundles::{DogBundle, GoblinBundle, OrcBundle},
    util::{random_table::RandomTable, urect_ext::URectExt},
};

#[derive(Event)]
pub enum Spawn {
    Dog(UVec2),
    Area { area: URect, depth: i32 },
}

macro_rules! spawners {
    ($($name:ident),+) => {
        paste! {
            $(fn $name(commands: &mut Commands) -> Entity {
                commands.spawn([<$name:camel Bundle>]::default()).id()
            })*
        }
    }
}
spawners!(goblin, orc);

pub fn spawn(
    mut commands: Commands,
    mut ev: EventReader<Spawn>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let rng = &mut *rng;
    for ev in ev.read() {
        match ev {
            Spawn::Dog(xy) => {
                let bundle = DogBundle {
                    energy: Energy::new(rng.gen_range(1..=500)),
                    position: Position::from(*xy),
                    ..Default::default()
                };
                commands.spawn(bundle).insert(Position::from(*xy));
            }

            Spawn::Area { area, depth } => {
                let spawn_table = RandomTable::<fn(&mut Commands) -> Entity>::new()
                    .add(goblin, 10)
                    .add(orc, 1 + depth);

                let spawnable_count = max(0, rng.gen_range(-2..=4 + depth)) as usize;
                for xy in area
                    .point_set()
                    .iter()
                    .choose_multiple(rng, spawnable_count)
                {
                    if let Some(spawner) = spawn_table.roll(rng) {
                        let entity = spawner(&mut commands);
                        commands.entity(entity).insert(Position::from(*xy));
                    }
                }
            }
        }
    }
}
