use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

use bsp::{BspConfig, BspMapBuilder};
//use crate::mapgen::cellular_automata::{CellularAutomataMapBuilder, DefaultCellularAutomataConfig};
//use crate::mapgen::drunkards_walk::DrunkardsWalkMapBuilder;
//use crate::mapgen::maze::MazeMapBuilder;
//use crate::mapgen::simple::SimpleMapBuilder;
use crate::plugins::{map::resources::Map, spawner::events::Spawn};

mod bsp;
//mod cellular_automata;
mod common;
//mod drunkards_walk;
//mod maze;
//mod simple;
//pub mod spawner;

struct SnapshotManager {
    snapshots: VecDeque<Map>,
}

impl SnapshotManager {
    #[must_use]
    fn new() -> Self {
        SnapshotManager {
            snapshots: VecDeque::new(),
        }
    }

    fn record_snapshot(&mut self, map: Map) {
        if cfg!(feature = "visualize-mapgen") {
            self.snapshots.push_back(map);
        }
    }

    fn get_snapshots(&self) -> VecDeque<Map> {
        self.snapshots.clone()
    }
}

pub trait MapBuilder<RNG: Rng> {
    fn build_map(&mut self, rng: &mut RNG);
    fn spawn_entities(&self, ev_spawn: EventWriter<Spawn>, rng: &mut RNG);
    fn get_starting_position(&self) -> UVec2;
    fn get_snapshots(&self) -> VecDeque<Map>;
    fn get_map(&self) -> Map;
}

pub fn random_builder<RNG: Rng>(
    rng: &mut RNG,
    width: u32,
    height: u32,
    new_depth: u32,
) -> Box<dyn MapBuilder<RNG>> {
    match rng.gen_range(1..=6) {
        x if x <= 3 => Box::new(BspMapBuilder::new(
            width,
            height,
            new_depth,
            BspConfig::dungeon(),
        )),
        _ => Box::new(BspMapBuilder::new(
            width,
            height,
            new_depth,
            BspConfig::interior(),
        )),
        /*
        3 => Box::new(CellularAutomataMapBuilder::new(
            width,
            height,
            new_depth,
            Box::new(DefaultCellularAutomataConfig),
        )),
        4 => Box::new(DrunkardsWalkMapBuilder::new(width, height, new_depth)),
        5 => Box::new(MazeMapBuilder::new(width, height, new_depth)),
        _ => Box::new(SimpleMapBuilder::new(width, height, new_depth)),
        */
    }
}
